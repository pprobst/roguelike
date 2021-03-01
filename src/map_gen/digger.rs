use super::{
    common::{create_h_tunnel_room, create_room, create_v_tunnel_room},
    room::Operations,
    CustomRegion, Map, Room, TileType,
};
use crate::utils::directions::*;
use bracket_lib::prelude::{DistanceAlg, Point, RandomNumberGenerator};

/*
 *
 * digger.rs
 * ---------
 * The digger/tunneler algorithm, based on:
 * http://www.roguebasin.com/index.php?title=Dungeon-Building_Algorithm
 *
 */

#[allow(dead_code)]
pub struct Digger<'a> {
    region: &'a CustomRegion,
    rooms: Vec<Room>,
    min_size: u8,
    max_size: u8,
    num_features: i32,
    peripheral: bool,
}

#[allow(dead_code)]
impl<'a> Digger<'a> {
    pub fn new(region: &'a CustomRegion, min_size: u8, max_size: u8, num_features: i32) -> Self {
        Self {
            region,
            rooms: vec![],
            min_size,
            max_size,
            num_features,
            peripheral: false,
        }
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        self.rooms.clone()
    }

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        // Create initial room on the center of the map.
        let center = self.region.get_center();
        let wi = rng.range(self.min_size, self.max_size) as i32;
        let hi = wi as i32;
        let initial_room = Room::with_size(center.x - 5, center.y - 5, wi, hi);
        create_room(map, initial_room, TileType::Floor);
        self.rooms.push(initial_room);
        self.gen_feature(map, rng);
    }

    fn add_feature(&mut self, map: &mut Map, room: Room, rng: &mut RandomNumberGenerator) -> bool {
        let w = rng.range(self.min_size, self.max_size);
        let h = rng.range(w, self.max_size);
        let dir = get_random_dir();
        let ndir = dir.clone();

        let mut pt = room.get_wall(map, dir);
        let room_gap = rng.range(3, 10);

        match ndir {
            NORTH => {
                pt.y -= (h + room_gap) as i32;
            }
            EAST => {
                pt.x += (room_gap) as i32;
            }
            SOUTH => {
                pt.y += (room_gap) as i32;
            }
            _ => {
                pt.x -= (w + room_gap) as i32;
            }
        }

        let new_room = Room::with_size(pt.x, pt.y, w as i32, h as i32);

        for r in self.rooms.iter() {
            if new_room.intersect(r)
                || !self.region.in_bounds(Point {
                    x: new_room.x1,
                    y: new_room.y1,
                })
                || !self.region.in_bounds(Point {
                    x: new_room.x2,
                    y: new_room.y2,
                })
            {
                return false;
            }
        }

        self.rooms.push(new_room);
        create_room(map, new_room, TileType::Floor);
        self.connect_rooms(map, room, new_room, rng);
        return true;
    }

    fn gen_feature(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let num_features = 0;
        let repeat = self.num_features * 3; // max number of iterations

        // Main, bigger rooms.
        self.gen_feature_loop(num_features, repeat, map, rng);

        self.min_size /= 2;
        self.max_size /= 2;

        // Peripheric, smaller rooms.
        self.peripheral = true;
        self.gen_feature_loop(num_features, repeat, map, rng);
    }

    fn gen_feature_loop(
        &mut self,
        mut num_features: i32,
        mut repeat: i32,
        map: &mut Map,
        rng: &mut RandomNumberGenerator,
    ) {
        let mut prev_idx = 0;
        while num_features <= self.num_features {
            repeat -= 1;
            if repeat <= 0 {
                break;
            }
            for _i in num_features..self.num_features {
                let idx = rng.range(0, self.rooms.len());
                if idx == prev_idx && self.rooms.len() > 1 {
                    continue;
                }
                prev_idx = idx;
                if self.add_feature(map, self.rooms[idx], rng) {
                    repeat += 1;
                    num_features += 1;
                }
            }
        }
    }

    fn connect_rooms(
        &mut self,
        map: &mut Map,
        room1: Room,
        room2: Room,
        rng: &mut RandomNumberGenerator,
    ) {
        let borders1 = room1.get_borders(map);
        let borders2 = room2.get_borders(map);

        let mut room_c = Point::new(0, 0);
        let mut other_c = Point::new(0, 0);

        let mut shortest_dist = DistanceAlg::Pythagoras.distance2d(borders1[0], borders2[0]);
        for b1 in borders1.iter() {
            for b2 in borders2.iter() {
                let d = DistanceAlg::Pythagoras.distance2d(*b1, *b2);
                if d < shortest_dist {
                    room_c = *b1;
                    other_c = *b2;
                    shortest_dist = d;
                }
            }
        }

        let mut size = rng.range(2, 4) as i32;
        if self.peripheral {
            size = 1;
        }

        match rng.range(0, 2) {
            0 => {
                if room_c.x <= other_c.x {
                    self.rooms.push(create_h_tunnel_room(
                        map,
                        room_c.x,
                        other_c.x,
                        room_c.y,
                        size,
                        TileType::Floor,
                    ));
                } else {
                    self.rooms.push(create_h_tunnel_room(
                        map,
                        other_c.x,
                        room_c.x,
                        room_c.y,
                        size,
                        TileType::Floor,
                    ));
                }

                if room_c.y <= other_c.y {
                    self.rooms.push(create_v_tunnel_room(
                        map,
                        room_c.y,
                        other_c.y,
                        other_c.x,
                        size,
                        TileType::Floor,
                    ));
                } else {
                    self.rooms.push(create_v_tunnel_room(
                        map,
                        other_c.y,
                        room_c.y,
                        other_c.x,
                        size,
                        TileType::Floor,
                    ));
                }
            }
            _ => {
                if room_c.y <= other_c.y {
                    self.rooms.push(create_v_tunnel_room(
                        map,
                        room_c.y,
                        other_c.y,
                        room_c.x,
                        size,
                        TileType::Floor,
                    ));
                } else {
                    self.rooms.push(create_v_tunnel_room(
                        map,
                        other_c.y,
                        room_c.y,
                        room_c.x,
                        size,
                        TileType::Floor,
                    ));
                }

                if room_c.x <= other_c.x {
                    self.rooms.push(create_h_tunnel_room(
                        map,
                        room_c.x,
                        other_c.x,
                        other_c.y,
                        size,
                        TileType::Floor,
                    ));
                } else {
                    self.rooms.push(create_h_tunnel_room(
                        map,
                        other_c.x,
                        room_c.x,
                        other_c.y,
                        size,
                        TileType::Floor,
                    ));
                }
            }
        }
    }
}
