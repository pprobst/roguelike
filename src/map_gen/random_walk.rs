use super::{CustomRegion, Map, Position, Tile, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * random_walk.rs
 * --------------
 * Random Walk algorithm, with some nice additions.
 *
 * http://www.roguebasin.com/index.php?title=Random_Walk_Cave_Generation
 *
 */

#[derive(Clone)]
struct Walker {
    life: i32,
    pos: Position,
}

// percent >= 0.4 if walkers start on random positions.
// percent <= 0.25 if walkers start on the center.
// Walkers that can only move orthogonally produce neater dungeons,
// while allowing diagonal movement produces chaotic dungeons.
#[allow(dead_code)]
pub struct RandomWalker<'a> {
    region: &'a CustomRegion,
    percent: f32,
    grouped_walkers: bool,
    can_walk_diagonally: bool,
}

#[allow(dead_code)]
impl<'a> RandomWalker<'a> {
    pub fn new(
        region: &'a CustomRegion,
        percent: f32,
        grouped_walkers: bool,
        can_walk_diagonally: bool,
    ) -> Self {
        //pub fn new(percent: f32, grouped_walkers: bool, can_walk_diagonally: bool) -> Self {
        Self {
            region,
            percent,
            grouped_walkers,
            can_walk_diagonally,
        }
    }

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let mut n_floor_tiles = self
            .region
            .pos
            .iter()
            .filter(|p| map.is_floor(map.idx_pt(**p)))
            .count();
        let needed_floor_tiles = (self.percent * self.region.size as f32) as usize;
        let center = self.region.get_center();

        let max = 500;
        let mut n_walkers = 0;
        // While insufficient cells have been turned into floor, take one step in a random direction.
        // If the new map cell is wall, turn the new map cell into floor and increment the count of floor tiles.
        while n_floor_tiles < needed_floor_tiles && n_walkers < max {
            n_walkers += 1;
            let mut walker;
            if self.grouped_walkers {
                walker = Walker {
                    life: rng.range(200, 500),
                    pos: center,
                };
            } else {
                walker = Walker {
                    life: rng.range(200, 500),
                    pos: Position::new(
                        rng.range(self.region.x1, self.region.x2),
                        rng.range(self.region.y1, self.region.y2),
                    ),
                };
            }
            //println!("{}", n_walkers);
            while walker.life > 0 {
                let idx = map.idx(walker.pos.x, walker.pos.y);
                if self.region.in_bounds(walker.pos) {
                    let new_dir = rng.range(0, 8);
                    match new_dir {
                        0 => {
                            walker.pos += EAST;
                        }
                        1 => {
                            walker.pos += WEST;
                        }
                        2 => {
                            walker.pos += NORTH;
                        }
                        3 => {
                            walker.pos += SOUTH;
                        }
                        4 => {
                            if self.can_walk_diagonally {
                                walker.pos += NORTHEAST;
                            }
                        }
                        5 => {
                            if self.can_walk_diagonally {
                                walker.pos += NORTHWEST;
                            }
                        }
                        6 => {
                            if self.can_walk_diagonally {
                                walker.pos += SOUTHEAST;
                            }
                        }
                        _ => {
                            if self.can_walk_diagonally {
                                walker.pos += SOUTHWEST;
                            }
                        }
                    }
                    if map.tiles[idx].ttype == TileType::Wall {
                        map.tiles[idx] = Tile::floor();
                        n_floor_tiles += 1;
                    }
                }
                walker.life -= 1;
            }
        }
        //println!("Total walkers: {}", _n_walkers);
    }
}
