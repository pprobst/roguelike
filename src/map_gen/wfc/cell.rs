use super::TileType;
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MapTile {
    pub idx: usize,
    pub pattern: Vec<TileType>,
    pub compatible: Vec<(usize, Direction)>, // overlaps with MapTile of idx usize at Direction
    pub size: i32,
}

impl MapTile {
    /// Returns compatible tile indexes on a given direction relative to this tile.
    pub fn get_compatible_dir(&self, dir: Direction) -> Vec<usize> {
        let mut compats: Vec<usize> = Vec::new();
        for c in self.compatible.iter() {
            if c.1 == dir {
                compats.push(c.0);
            }
        }
        compats
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileEnablerCount {
    // EAST, WEST, NORTH, SOUTH
    pub by_direction: [usize; 4],
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub possible: Vec<bool>,
    pub possible_tiles: Vec<usize>,
    sum_possible_weights: f32,
    sum_possible_weights_log: f32,
    pub entropy_noise: f32,
    pub collapsed: bool,
    pub enabler_count: Vec<TileEnablerCount>,
}

impl Cell {
    pub fn new(num_tiles: usize, entropy_noise: f32) -> Self {
        Self {
            possible: vec![true; num_tiles], // Initially all tiles are possible on a cell
            possible_tiles: (0..num_tiles).collect(),
            sum_possible_weights: 0.0,
            sum_possible_weights_log: 0.0,
            entropy_noise,
            collapsed: false,
            enabler_count: Vec::new(),
        }
    }

    /// For each possible tile on this cell, counts how many compatible tiles (enablers) it
    /// has in all 4 possible directions.
    pub fn initial_enabler_count(&mut self, maptiles: Vec<MapTile>) {
        for (i, _p) in self.possible.iter().enumerate() {
            let mut counts = TileEnablerCount {
                by_direction: [0, 0, 0, 0],
            };
            counts.by_direction[0] += maptiles[i].get_compatible_dir(EAST).len();
            counts.by_direction[1] += maptiles[i].get_compatible_dir(WEST).len();
            counts.by_direction[2] += maptiles[i].get_compatible_dir(NORTH).len();
            counts.by_direction[3] += maptiles[i].get_compatible_dir(SOUTH).len();
            self.enabler_count.push(counts);
        }
    }

    /// Calculates the entropy (negated).
    pub fn entropy(&self) -> f32 {
        let entropy = self.sum_possible_weights.log2()
            - (self.sum_possible_weights_log / self.sum_possible_weights as f32);
        return entropy;
    }

    /// Removes a map tile (pattern index) from the list of possible tiles, and
    /// updates the sums of possible weights for the Entropy calculation.
    pub fn remove_tile(&mut self, tile_idx: usize, freq: &HashMap<usize, f32>) {
        if !self.possible[tile_idx] {
            return;
        }
        self.possible[tile_idx] = false;
        self.possible_tiles.retain(|t| *t != tile_idx);

        let f = freq.get(&tile_idx).unwrap();
        self.sum_possible_weights -= f;
        self.sum_possible_weights_log -= f * f.log2();
    }

    /// Adds up the relative frequencies of all possible tiles.
    /// Also calculates the log sum.
    pub fn total_possible_tile_freq(&mut self, freq: &HashMap<usize, f32>) {
        let mut total = 0.0;
        let mut total_log = 0.0;
        for idx in self.possible_tiles.iter() {
            if freq.contains_key(idx) {
                let freq_hint = freq.get(idx).unwrap();
                total += freq_hint;
                total_log += freq_hint * freq_hint.log2();
            }
        }
        self.sum_possible_weights = total;
        self.sum_possible_weights_log = total_log;
        //println!("sum:     {}", self.sum_possible_weights);
        //println!("sum log: {}", self.sum_possible_weights_log);
        //println!("noise: {}", self.entropy_noise);
    }

    /// Selects a tile based on the frequency table.
    pub fn choose_tile(
        &self,
        freq: &HashMap<usize, f32>,
        rng: &mut RandomNumberGenerator,
    ) -> usize {
        let mut remain = rng.rand::<f32>() * self.sum_possible_weights;

        for idx in self.possible_tiles.iter() {
            let weight = *freq.get(idx).unwrap();
            if remain >= weight {
                remain -= weight;
            } else {
                return *idx;
            }
        }

        unreachable!("sum_possible_weights was inconsistent!");
    }

    /// Checks if there's a contradiction in the current cell.
    /// That is, not a single tile is possible for this cell.
    pub fn contradiction_check(&self) -> bool {
        self.possible.iter().all(|d| *d == false)
    }
}
