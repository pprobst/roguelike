use super::{CustomRegion, Map, Point, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::collections::HashMap;

mod common;
use common::*;
mod cell;
use cell::*;
mod wave;
use wave::*;

/*
 * This project contains the WaveFunctionCollapse (WFC) algorithm.
 * https://github.com/mxgmn/WaveFunctionCollapse
 *
 * This is NOT meant to be the fastest WFC algorithm possible; stevebob/wfc is plenty
 * fast already. Our point here is to LEARN and USE WFC, comparing it with traditional
 * PCG methods.
 *
 * For reference, I used the following resources:
 * - https://gridbugs.org/wave-function-collapse/ (mainly)
 * - https://frame.42yeah.casa/2020/01/30/wfc.html
 * - https://www.youtube.com/watch?v=ws4r3wLPNSE&list=PLcRSafycjWFeKAS40OdIvhL7j-vsgE3eg
 * - https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/
 * - https://github.com/BigYvan/Wave-Function-Collapse
 *
 */

#[derive(Debug, Clone)]
pub struct WaveFunctionCollapse<'a> {
    tile_size: i32,
    patterns: Vec<Vec<TileType>>,
    constraints: Vec<MapTile>,
    frequencies: HashMap<usize, f32>,
    region: &'a CustomRegion,
    mix_match: bool, // Used for larger inputs.
}

#[allow(dead_code)]
impl<'a> WaveFunctionCollapse<'a> {
    pub fn new(tile_size: i32, region: &'a CustomRegion, mix_match: bool) -> Self {
        Self {
            tile_size,
            patterns: Vec::new(),
            constraints: Vec::new(),
            frequencies: HashMap::new(),
            region,
            mix_match,
        }
    }

    /// Runs the whole WFC algorithm.
    pub fn generate(
        &mut self,
        output_map: &mut Map,
        input_map: &Map,
        input_x: i32,
        input_y: i32,
        rng: &mut RandomNumberGenerator,
    ) -> bool {
        self.build_patterns(input_map, input_x, input_y);
        let patterns = self.patterns.clone();
        //println!("Patterns: {}", patterns.len());
        //map.tiles = vec![Tile::woodenfloor(); (map.width * map.height) as usize];

        deduplicate(&mut self.patterns);

        self.build_constraints(); // patterns + adjacency rules
        let constraints = self.constraints.clone();
        self.compute_frequencies(&patterns, &constraints); // frequency hints

        let out_width = self.region.width / self.tile_size;
        let out_height = self.region.height / self.tile_size;
        let out_size = out_width * out_height;

        // Initialize Cells.
        let cells = self.init_cells(out_size, rng);

        // Initialize Wave.
        let mut wave = Wave::new(cells, constraints, out_width, out_height);
        wave.init_entropy_queue();

        // Run Wave.
        if !self.run_wave(&mut wave, rng) {
            return false;
        }

        // Generate the output.
        //map.tiles = vec![Tile::wall(); (output_map.width * output_map.height) as usize];
        self.generate_output(wave, output_map);

        true
    }

    /// Initialize all the cells.
    fn init_cells(&self, n_cells: i32, rng: &mut RandomNumberGenerator) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();
        for _i in 0..n_cells {
            let noise = rng.rand::<f32>() / 10000.0; // Random noise to break entropy ties
            let cell = Cell::new(self.constraints.len(), noise);
            cells.push(cell);
        }
        for cell in cells.iter_mut() {
            cell.total_possible_tile_freq(&self.frequencies);
            cell.initial_enabler_count(self.constraints.clone());
        }
        cells
    }

    /// Runs the core WFC solver.
    fn run_wave(&mut self, wave: &mut Wave, rng: &mut RandomNumberGenerator) -> bool {
        while wave.uncollapsed_cells > 0 {
            let next_coord = wave.choose_next_cell();
            wave.collapse_cell_at(next_coord, &self.frequencies, rng);
            if !wave.propagate(&self.frequencies) {
                return false;
            }
            wave.uncollapsed_cells -= 1;
        }
        true
    }

    /// Translates the information on the wave to the map, thus generating the output.
    fn generate_output(&mut self, wave: Wave, map: &mut Map) {
        for (i, cell) in wave.cells.iter().enumerate() {
            let cell_x = i as i32 % wave.out_width;
            let cell_y = i as i32 / wave.out_width;

            let x1 = self.region.x1 + cell_x * self.tile_size;
            let x2 = self.region.x1 + (cell_x + 1) * self.tile_size;
            let y1 = self.region.y1 + cell_y * self.tile_size;
            let y2 = self.region.y1 + (cell_y + 1) * self.tile_size;

            let mut j: usize = 0;
            for y in y1..y2 {
                for x in x1..x2 {
                    let map_idx = map.idx(x, y);
                    let tile_idx = cell.possible_tiles[0]; // Only one because the cell is collapsed.
                    let tile = self.constraints[tile_idx].pattern[j];
                    map.paint_tile(map_idx, tile);
                    j += 1;
                }
            }
        }
    }

    /// Builds tiles of size tile_size*tile_size from the given map cells.
    fn build_patterns(&mut self, map: &Map, input_x: i32, input_y: i32) {
        self.patterns.clear();
        // Navigate the coordinates of each tile.
        let y1 = if !self.mix_match {
            input_y
        } else {
            input_y / self.tile_size
        };
        let x1 = if !self.mix_match {
            input_x
        } else {
            input_x / self.tile_size
        };
        for ty in 0..y1 {
            for tx in 0..x1 {
                let start = if !self.mix_match {
                    Point::new(tx, ty)
                } else {
                    Point::new(tx * self.tile_size, ty * self.tile_size)
                };
                let end = if !self.mix_match {
                    Point::new(tx + self.tile_size, ty + self.tile_size)
                } else {
                    Point::new((tx + 1) * self.tile_size, (ty + 1) * self.tile_size)
                };
                //println!("Start: {:?}, End: {:?}", start, end);
                let normal_pattern = self.get_pattern(map, start, end, "normal");
                let vert_pattern = self.get_pattern(map, start, end, "vertical");
                let horiz_pattern = self.get_pattern(map, start, end, "horizontal");
                let verthoriz_pattern = self.get_pattern(map, start, end, "both");
                //let inverted_pattern = self.get_pattern(map, start, end, "invert");
                self.patterns.push(normal_pattern);
                self.patterns.push(vert_pattern);
                self.patterns.push(horiz_pattern);
                self.patterns.push(verthoriz_pattern);
                //self.patterns.push(inverted_pattern);
            }
        }
    }

    /// Returns a pattern (reflected or not) taken from the input.
    fn get_pattern(&mut self, map: &Map, start: Point, end: Point, rot: &str) -> Vec<TileType> {
        let mut pattern: Vec<TileType> = Vec::new();
        for y in start.y..end.y {
            for x in start.x..end.x {
                let idx;
                match rot {
                    "vertical" => {
                        idx = map.idx(x, end.y - (y + 1));
                    }
                    "horizontal" => {
                        idx = map.idx(end.x - (x + 1), y);
                    }
                    "both" => {
                        idx = map.idx(end.x - (x + 1), end.y - (y + 1));
                    }
                    "invert" => {
                        if map.in_map_bounds_xy(y, x) {
                            idx = map.idx(y, x);
                        } else {
                            idx = map.idx(x, y);
                        }
                        //idx = map.idx(end.y - (y + 1), end.x - (x + 1));
                    }
                    _ => {
                        idx = map.idx(x, y);
                    }
                }
                pattern.push(map.tiles[idx].ttype);
            }
        }
        pattern
    }

    /// Compute the relative frequencies of each tile.
    fn compute_frequencies(&mut self, patterns: &Vec<Vec<TileType>>, constraints: &Vec<MapTile>) {
        // Calculate absolute frequencies.
        for tile in constraints.iter() {
            for p in patterns.iter() {
                if tile.pattern == *p {
                    *self.frequencies.entry(tile.idx).or_insert(0.0) += 1.0;
                }
            }
        }
        // Update absolute frequencies to relative frequencies.
        let total: f32 = self.frequencies.values().sum();
        for v in self.frequencies.values_mut() {
            *v /= total;
        }
    }

    /// Build the "contraints", that is, the possible tiles and their compatibilities.
    /// In other other, we're building each "map tile" with its adjacency rules.
    fn build_constraints(&mut self) {
        for (i, p1) in self.patterns.iter().enumerate() {
            let mut map_tile = MapTile {
                idx: i, // Each tile has an index
                pattern: p1.to_vec(),
                compatible: Vec::new(),
                size: self.tile_size,
            };
            //println!("{:?}", p1);
            for (j, p2) in self.patterns.iter().enumerate() {
                //if p1 == p2 { continue; }
                if self.is_compatible(p1, p2, EAST) {
                    map_tile.compatible.push((j, EAST));
                    //println!("{} compat with {:?} NORTH", i, j);
                }
                if self.is_compatible(p1, p2, WEST) {
                    map_tile.compatible.push((j, WEST));
                    //println!("{} compat with {:?} SOUTH", i, j);
                }
                if self.is_compatible(p1, p2, NORTH) {
                    map_tile.compatible.push((j, NORTH));
                    //println!("{} compat with {:?} EAST", i, j);
                }
                if self.is_compatible(p1, p2, SOUTH) {
                    map_tile.compatible.push((j, SOUTH));
                    //println!("{} compat with {:?} WEST", i, j);
                }
            }
            self.constraints.push(map_tile);
        }
    }

    /// Checks if there is overlap between two patterns.
    fn is_compatible(&self, p1: &Vec<TileType>, p2: &Vec<TileType>, dir: Direction) -> bool {
        let xmin = if dir.delta_x < 0 { 0 } else { dir.delta_x };
        let xmax = if dir.delta_x < 0 {
            dir.delta_x + self.tile_size as i8
        } else {
            self.tile_size as i8
        };
        let ymin = if dir.delta_y < 0 { 0 } else { dir.delta_y };
        let ymax = if dir.delta_y < 0 {
            dir.delta_y + self.tile_size as i8
        } else {
            self.tile_size as i8
        };

        // Iterate on every symbol in the intersection of the two patterns.
        // If any symbol is different, return false.
        for y in ymin..ymax {
            for x in xmin..xmax {
                let p1_pos = Point::new(x, y);
                let offset = p1_pos - dir;
                if p1[tile_idx(self.tile_size, p1_pos.x, p1_pos.y)]
                    != p2[tile_idx(self.tile_size, offset.x, offset.y)]
                {
                    return false;
                }
            }
        }
        true
    }
}
