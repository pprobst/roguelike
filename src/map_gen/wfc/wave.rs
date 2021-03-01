use super::{Cell, MapTile, Point};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
pub struct Wave {
    pub cells: Vec<Cell>,
    pub uncollapsed_cells: usize,
    maptiles: Vec<MapTile>,
    entropy_queue: BinaryHeap<CoordEntropy>,
    tile_removals: Vec<RemovalUpdate>, // stack
    pub out_width: i32,
    pub out_height: i32,
}

#[allow(dead_code)]
impl Wave {
    pub fn new(cells: Vec<Cell>, maptiles: Vec<MapTile>, out_width: i32, out_height: i32) -> Self {
        let cells_len = cells.len(); // or out_width * out_height
        Self {
            cells,
            uncollapsed_cells: cells_len,
            maptiles,
            entropy_queue: BinaryHeap::new(),
            tile_removals: Vec::new(),
            out_height,
            out_width,
        }
    }

    /// Initialized the entropy queue.
    pub fn init_entropy_queue(&mut self) {
        for y in 0..self.out_height {
            for x in 0..self.out_width {
                let idx = self.cell_at(x, y);
                let cell = &self.cells[idx];
                self.entropy_queue.push(CoordEntropy {
                    entropy: Entropy {
                        entropy: cell.entropy(),
                        noise: cell.entropy_noise,
                    },
                    coord: Point::new(x, y),
                });
            }
        }
    }

    /// Returns the cell at (x, y) on the wave.
    fn cell_at(&self, x: i32, y: i32) -> usize {
        (y as usize * self.out_width as usize) + x as usize
    }

    /// Returns true if (x, y) is in the bounds of the wave; false otherwise.
    fn in_bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.out_width && y >= 0 && y < self.out_height
    }

    /// Given a tile index, returns all the compatible tiles it has.
    pub fn get_compatible_dir(&self, idx: usize, dir: Direction) -> Vec<usize> {
        self.maptiles[idx].get_compatible_dir(dir)
    }

    /// Select the next cell to collapse and return its coordinate in the wave.
    pub fn choose_next_cell(&mut self) -> Point {
        while let Some(entropy_coord) = self.entropy_queue.pop() {
            let idx = self.cell_at(entropy_coord.coord.x, entropy_coord.coord.y);
            let cell = &self.cells[idx];
            if !cell.collapsed {
                return entropy_coord.coord;
            }
        }
        unreachable!("entropy_queue is empty!");
    }

    /// Collapses a cell at a given point.
    /// That is, remove all the possibilities except the only possible one.
    pub fn collapse_cell_at(
        &mut self,
        pt: Point,
        freq: &HashMap<usize, f32>,
        rng: &mut RandomNumberGenerator,
    ) {
        let idx = self.cell_at(pt.x, pt.y);
        let mut cell = &mut self.cells[idx];
        let locked_tile = cell.choose_tile(freq, rng);

        cell.collapsed = true;

        let possibles = cell.possible_tiles.clone();
        for idx in possibles.iter() {
            if *idx != locked_tile {
                cell.remove_tile(*idx, freq);
                self.tile_removals.push(RemovalUpdate {
                    tile: *idx,
                    coord: pt,
                });
            }
        }
    }

    /// Keeps propagating consequences until there are none (think like it's a sudoku game).
    pub fn propagate(&mut self, freq: &HashMap<usize, f32>) -> bool {
        let directions = [EAST, WEST, NORTH, SOUTH];

        while let Some(removal_update) = self.tile_removals.pop() {
            //println!("NEW REMOVAL");
            // Iterate through each adjacent tile to the current one.
            for i in 0..4 {
                let dir = directions[i];

                let neighbor_coord = removal_update.coord + dir;

                // Skip if coordinates are outside the bounds.
                if !self.in_bound(neighbor_coord.x, neighbor_coord.y) {
                    continue;
                }

                let neighbor_idx = self.cell_at(neighbor_coord.x, neighbor_coord.y);

                // Skip if the neighbor cell is already collapsed.
                if self.cells[neighbor_idx].collapsed {
                    continue;
                }

                let compatible_tiles = self.get_compatible_dir(removal_update.tile, dir);
                let neighbor_cell = &mut self.cells[neighbor_idx];

                for compat in compatible_tiles.iter() {
                    let j = opposite_idx(i); // Opposite direction to i

                    if neighbor_cell.enabler_count[*compat].by_direction[j] == 1 {
                        if neighbor_cell.possible[*compat] {
                            neighbor_cell.remove_tile(*compat, freq);
                            if neighbor_cell.contradiction_check() {
                                println!("Contradiction!");
                                return false;
                            }
                            self.entropy_queue.push(CoordEntropy {
                                entropy: Entropy {
                                    entropy: neighbor_cell.entropy(),
                                    noise: neighbor_cell.entropy_noise,
                                },
                                coord: neighbor_coord,
                            });
                            self.tile_removals.push(RemovalUpdate {
                                tile: *compat,
                                coord: neighbor_coord,
                            });
                        }
                    }
                    neighbor_cell.enabler_count[*compat].by_direction[j] -= 1;
                }
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct RemovalUpdate {
    tile: usize,
    coord: Point,
}

#[derive(Debug, PartialEq, Clone)]
struct Entropy {
    entropy: f32,
    noise: f32,
}

impl Eq for Entropy {}

impl PartialOrd for Entropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.entropy.partial_cmp(&other.entropy) {
            Some(Ordering::Equal) => self.noise.partial_cmp(&other.noise),
            other_ordering => other_ordering,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct CoordEntropy {
    coord: Point,
    entropy: Entropy,
}

impl PartialOrd for CoordEntropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl Ord for CoordEntropy {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            return Ordering::Less;
        }
        if self == other {
            return Ordering::Equal;
        }
        return Ordering::Greater;
    }
}
