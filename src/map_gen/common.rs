use super::{region::Operations, CustomRegion, Map, Region, Room, Tile, TileType, Tunnel};
use crate::utils::directions::*;
use bracket_lib::prelude::{DistanceAlg, Point, RandomNumberGenerator};
use std::cmp;

/*
 *
 * common.rs
 * ---------
 * Contains some general code that can be used by various map generators.
 * https://github.com/Vinatorul/dungeon-generator-rs/blob/master/src/bsp_generator.rs
 *
 */

#[allow(dead_code)]
pub fn rect_region(x1: i32, y1: i32, w: i32, h: i32) -> Vec<Point> {
    let x2 = x1 + w;
    let y2 = y1 + h;
    let mut points = Vec::new();

    for y in y1..y2 {
        for x in x1..x2 {
            points.push(Point::new(x, y));
        }
    }

    points
}

#[allow(dead_code)]
pub fn circular_region(x1: i32, y1: i32, radius: i32) -> Vec<Point> {
    let diameter = radius * 2;
    let x2 = x1 + diameter;
    let y2 = y1 + diameter;
    let r = (x2 - x1) / 2;
    let center = Point::new((x1 + x2) / 2, (y1 + y2) / 2);
    let mut points = Vec::new();

    for y in y1..y2 {
        for x in x1..x2 {
            let d = DistanceAlg::Pythagoras.distance2d(center, Point::new(x, y));
            if d < r as f32 {
                points.push(Point::new(x, y));
            }
        }
    }

    points
}

#[allow(dead_code)]
pub fn region_width(region: &Vec<Point>) -> i32 {
    let max = region.iter().max_by_key(|p| p.x).unwrap().x;
    let min = region.iter().min_by_key(|p| p.x).unwrap().x;
    max - min
}

#[allow(dead_code)]
pub fn region_height(region: &Vec<Point>) -> i32 {
    let max = region.iter().max_by_key(|p| p.y).unwrap().y;
    let min = region.iter().min_by_key(|p| p.y).unwrap().y;
    max - min
}

#[allow(dead_code)]
/// Creates a rectangular room and returns it.
pub fn create_room(map: &mut Map, room: Room, ttype: TileType) -> Room {
    for y in (room.y1 + 1)..room.y2 {
        for x in (room.x1 + 1)..room.x2 {
            let idx = map.idx(x, y);
            map.paint_tile(idx, ttype);
        }
    }

    room
}

#[allow(dead_code)]
/// Creates a circular room and returns it.
pub fn create_circular_room(map: &mut Map, room: Room, ttype: TileType) -> Room {
    let r = i32::min(room.x2 - room.x1, room.y2 - room.y1) as f32 / 2.0;
    let cp = Point::from(room.center());

    for y in (room.y1 + 1)..room.y2 {
        for x in (room.x1 + 1)..room.x2 {
            let d = DistanceAlg::Pythagoras.distance2d(cp, Point::new(x, y));
            if d < r {
                let idx = map.idx(x, y);
                map.paint_tile(idx, ttype);
            }
        }
    }

    room
}

#[allow(dead_code)]
/// Creates a horizontal tunnel (corridor) and returns it.
pub fn create_h_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32, size: i32) -> Tunnel {
    let mut tunnel = Vec::new();

    for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
        let mut idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
        if size > 1 {
            for i in 1..2 {
                idx = map.idx(x, y + i);
                map.paint_tile(idx, TileType::Floor);
                tunnel.push(idx);
            }
        }
    }

    tunnel
}

#[allow(dead_code)]
/// Creates a vertical tunnel and returns it.
pub fn create_v_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32, size: i32) -> Tunnel {
    let mut tunnel = Vec::new();
    for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
        let mut idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
        if size > 1 {
            for i in 1..size {
                idx = map.idx(x + i, y);
                map.paint_tile(idx, TileType::Floor);
                tunnel.push(idx);
            }
        }
    }

    tunnel
}

#[allow(dead_code)]
pub fn create_h_tunnel_room(
    map: &mut Map,
    x1: i32,
    x2: i32,
    y: i32,
    size: i32,
    ttype: TileType,
) -> Room {
    let left = cmp::min(x1, x2);
    let right = cmp::max(x1, x2);
    let top = y - 1;
    let bottom = y + 1;
    let room = Room::with_size(left, top, right - left + size - 1, bottom - top + 1);
    create_room(map, room, ttype);
    room
}

#[allow(dead_code)]
pub fn create_v_tunnel_room(
    map: &mut Map,
    y1: i32,
    y2: i32,
    x: i32,
    size: i32,
    ttype: TileType,
) -> Room {
    let top = cmp::min(y1, y2);
    let bottom = cmp::max(y1, y2);
    let left = x - 1;
    let right = x + 1;
    let room = Room::with_size(left, top, right - left + size - 1, bottom - top + 1);
    create_room(map, room, ttype);
    room
}

#[allow(dead_code)]
pub fn make_exact_tunnel(
    map: &mut Map,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    ttype: TileType,
    natural: bool,
) {
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.idx(x, y);
        if map.tiles[idx].ttype != TileType::ShallowWater
            && map.tiles[idx].ttype != TileType::DeepWater
        {
            map.paint_tile(idx, ttype);

            if natural {
                let mut rng = RandomNumberGenerator::new();
                let sign_x = rng.range(0, 3);
                let sign_y = rng.range(0, 3);
                let add_x = if sign_x < 1 { 1 } else { -1 };
                let add_y = if sign_y < 1 { 1 } else { -1 };
                if map.in_map_bounds_xy(x + add_x, y + add_y) {
                    let mut idx2 = map.idx(x + add_x, y + add_y);
                    if map.tiles[idx2].ttype != TileType::ShallowWater
                        && map.tiles[idx2].ttype != TileType::DeepWater
                    {
                        map.paint_tile(idx2, ttype);
                        let one_more = rng.range(0, 5);
                        if one_more < 1 && map.in_map_bounds_xy(x + (add_x * 2), y + (add_y * 2)) {
                            idx2 = map.idx(x + (add_x * 2), y + (add_y) * 2);
                            if map.tiles[idx2].ttype != TileType::ShallowWater
                                && map.tiles[idx2].ttype != TileType::DeepWater
                            {
                                map.paint_tile(idx2, ttype);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn make_lake(map: &mut Map, region: &CustomRegion, liquid: TileType, total_tiles: u32) {
    let mut rng = RandomNumberGenerator::new();

    let x = rng.range(region.x1, region.x2);
    let y = rng.range(region.y1, region.y2);

    let mut walker_pos = Point::new(x, y);
    let mut n_tiles = 0;
    let mut max_tries = total_tiles * 5;

    while n_tiles <= total_tiles && max_tries > 0 {
        if region.in_bounds(walker_pos) {
            let idx = map.idx_pt(walker_pos);
            match liquid {
                TileType::DeepWater => {
                    map.tiles[idx] = Tile::deep_water();
                    map.tiles[idx + 1] = Tile::deep_water();
                    map.tiles[idx - 1] = Tile::deep_water();
                }
                _ => {
                    map.tiles[idx] = Tile::shallow_water();
                    map.tiles[idx + 1] = Tile::shallow_water();
                    map.tiles[idx - 1] = Tile::shallow_water();
                }
            }
            let dir = get_random_dir();
            walker_pos += dir;
            n_tiles += 1;
        }
        max_tries -= 1;
    }
}

/// Counts how many neighbor tiles of a given type curr_pt has.
/// If moore == true, considers a Moore neighborhood (ortoghonal+diagonals neighbors).
/// If moore == false, considers a von Neumann neighborhood (orthogonal neighbors).
pub fn count_neighbor_tile(map: &Map, curr_pt: Point, tt: TileType, moore: bool) -> u8 {
    let mut counter = 0;
    for i in 0..8 {
        if !moore && i >= 4 {
            break;
        }
        let pt = curr_pt + dir_idx(i);
        if map.in_map_bounds(pt) {
            if map.tiles[map.idx_pt(pt)].ttype == tt {
                counter += 1;
            }
        }
    }
    counter
}

pub fn count_neighbor_tile_entity(
    map: &Map,
    curr_pt: Point,
    tt: Vec<TileType>,
    moore: bool,
) -> (u8, usize) {
    let mut counter = 0;

    if map.entities[map.idx_pt(curr_pt)] != None {
        counter += 1;
    }

    let mut dir = 8;
    for i in 0..8 {
        if !moore && i >= 4 {
            break;
        }
        let pt = curr_pt + dir_idx(i);
        if map.in_map_bounds(pt) {
            let idx = map.idx_pt(pt);
            if tt.contains(&map.tiles[idx].ttype) || map.entities[idx] != None {
                counter += 1;
                dir = i;
            }
        }
    }
    (counter, dir)
}

#[allow(dead_code)]
pub fn add_vegetation(map: &mut Map, region: &CustomRegion, trees: bool) {
    let mut rng = RandomNumberGenerator::new();
    for y in region.y1..region.y2 {
        for x in region.x1..region.x2 {
            let idx = map.idx(x, y);
            if !map.tiles[idx].block && !map.is_water(idx) {
                let mut chance = rng.range(0, 4);
                if chance < 2 {
                    let pt = map.idx_pos(idx);
                    let water_counter = count_neighbor_tile(map, pt, TileType::ShallowWater, false);
                    if water_counter >= 1 {
                        map.tiles[idx] = Tile::tallgrass();
                    } else {
                        chance = rng.range(0, 90);
                        if chance < 2 {
                            if rng.range(0, 10) <= 7 {
                                map.tiles[idx] = Tile::flower();
                            } else {
                                map.tiles[idx] = Tile::mushroom();
                            }
                        } else if chance < 70 {
                            if rng.range(0, 10) <= 1 {
                                map.tiles[idx] = Tile::grass3();
                            } else {
                                map.tiles[idx] = Tile::grass();
                            }
                        } else if chance < 85 {
                            if rng.range(0, 2) < 1 {
                                map.tiles[idx] = Tile::grass4();
                            } else {
                                map.tiles[idx] = Tile::grass2();
                            }
                        } else if trees {
                            map.tiles[idx] = Tile::tree();
                        }
                    }
                }
            }
        }
    }
}

/// Gets all the separated regions on a map.
pub fn get_all_regions(map: &Map, inside: &CustomRegion) -> Vec<Region> {
    //let w = map.width;
    //let h = map.height;
    let mut caves: Vec<Region> = Vec::new();
    let mut marked_map: Vec<bool> = vec![false; map.size as usize];

    let y1 = if inside.y1 == 0 { 1 } else { inside.y1 };
    let x1 = if inside.x1 == 0 { 1 } else { inside.x1 };
    for y in y1..inside.y2 - 1 {
        for x in x1..inside.x2 - 1 {
            let idx = map.idx(x, y);
            if !marked_map[idx] && (map.is_walkable(idx) || map.is_door(idx)) {
                let new_cave = get_region(idx, map);

                for idx in new_cave.iter() {
                    marked_map[*idx] = true;
                }

                caves.push(new_cave);
            }
        }
    }

    caves
}

/// Gets a single region from a map, given an initial index (flood fill).
pub fn get_region(start_idx: usize, map: &Map) -> Region {
    use std::collections::VecDeque;
    let mut region_tiles: Region = Vec::new();
    let mut marked_map: Vec<bool> = vec![false; map.size as usize];
    let mut queue: VecDeque<usize> = VecDeque::new();

    queue.push_back(start_idx);
    marked_map[start_idx] = true;

    while !queue.is_empty() {
        let tile = queue.pop_front().unwrap();
        region_tiles.push(tile);
        let pt = map.idx_pos(tile);
        for y in pt.y - 1..pt.y + 2 {
            for x in pt.x - 1..pt.x + 2 {
                let idx = map.idx(x, y);
                if map.in_map_bounds_xy(x, y) && (y == pt.y || x == pt.x) {
                    if !marked_map[idx] && (map.is_walkable(idx) || map.is_door(idx)) {
                        marked_map[idx] = true;
                        queue.push_back(idx);
                    }
                }
            }
        }
    }

    region_tiles
}

/// Connects with tunnels the selected regions.
pub fn connect_regions(map: &mut Map, regions: Vec<Region>, ttype: TileType, natural: bool) {
    if regions.len() <= 1 {
        return;
    }
    // Algorithm idea:
    // - get the two points (x, y) that are the closest between two caves
    // - make a tunnel between then
    let mut region_pts: Vec<Vec<Point>> = Vec::new();

    // Populate the vector cave_pts (same as before, but considering the
    // coordinates on the map instead of the index).
    for region in regions.iter() {
        let mut pts: Vec<Point> = Vec::new();
        for idx in region {
            let pt = map.idx_pos(*idx);
            if region.is_probably_edge(pt, map) {
                pts.push(pt);
            }
        }
        region_pts.push(pts);
    }

    for i in 0..region_pts.len() - 1 {
        let this_region = &region_pts[i];
        let other_region = &region_pts[i + 1];
        if other_region.len() == 0 || this_region.len() == 0 {
            return;
        }
        let mut shortest_dist = other_region.len();
        let mut this_idx = 0;
        let mut other_idx = 0;
        for j in 0..this_region.len() - 1 {
            for k in 0..other_region.len() - 1 {
                let d =
                    DistanceAlg::Pythagoras.distance2d(this_region[j], other_region[k]) as usize;
                if d < shortest_dist {
                    this_idx = j;
                    other_idx = k;
                    shortest_dist = d;
                }
            }
        }
        make_exact_tunnel(
            map,
            this_region[this_idx].x,
            this_region[this_idx].y,
            other_region[other_idx].x,
            other_region[other_idx].y,
            ttype,
            natural,
        );
    }
}

/// Makes map chaotic with a chance of floor_chance to change a tile to floor.
/// Used in mapgen algorithms that require a "chaotic map" like Cellular Automata.
pub fn make_chaotic(map: &mut Map, region: &CustomRegion, floor_chance: u8) {
    let mut rng = RandomNumberGenerator::new();

    for pt in region.pos.iter() {
        let idx = map.idx_pt(*pt);
        if rng.range(1, 101) <= floor_chance {
            map.tiles[idx] = Tile::floor();
        };
    }
}

pub fn apply_forest_theme(map: &mut Map, region: &CustomRegion) {
    for pt in region.pos.iter() {
        let idx = map.idx_pt(*pt);
        if map.is_wall(idx) {
            map.tiles[idx] = Tile::tree();
        }
    }
}

/// Adds doors to ROOMS of a map, given a certain chance.
pub fn add_doors(map: &mut Map, rooms: &Vec<Room>, chance: i32, rng: &mut RandomNumberGenerator) {
    if rooms.len() > 0 {
        let mut locs_vec: Vec<Vec<usize>> = Vec::new();
        //let mut r = rooms.unwrap().clone();
        let mut r = rooms.clone();
        r.retain(|a| a.width() >= 5);
        //r.sort_by(|a, b| a.x2.cmp(&b.x2));
        for room in r.iter() {
            if rng.range(0, 100) >= chance {
                continue;
            }
            let mut locs: Vec<usize> = Vec::new();
            for y in room.y1..=room.y2 {
                for x in room.x1..=room.x2 {
                    if !map.in_map_bounds_xy(x, y) || !map.in_map_bounds_neighbors(Point::new(x, y))
                    {
                        continue;
                    }
                    let idx = map.idx(x, y);
                    if map.is_floor(idx) {
                        if y == room.y1 || y == room.y2 || x == room.x1 || x == room.x2 {
                            locs.push(idx);
                        }
                    }
                }
            }
            locs_vec.push(locs);
        }

        for locs in locs_vec.iter() {
            if locs.len() <= 5 {
                for loc in locs.iter() {
                    let pt = map.idx_pos(*loc);
                    let door_count = count_neighbor_tile(map, pt, TileType::ClosedDoor, true);
                    let wall_count = count_neighbor_tile(map, pt, TileType::Wall, false);
                    if door_count >= 2 {
                        continue;
                    }
                    if wall_count >= 3 && door_count < 2 {
                        continue;
                    }
                    map.tiles[*loc] = Tile::closed_door();
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn can_place_door(map: &mut Map, idx: usize) -> bool {
    if !map.in_map_bounds_idx(idx)
        || !map.in_map_bounds_idx(idx - 1)
        || !map.in_map_bounds_idx(idx + 1)
        || !map.in_map_bounds_idx(idx - map.width as usize)
        || !map.in_map_bounds_idx(idx + map.width as usize)
    {
        return false;
    }

    // North/South
    if (map.is_floor(idx))
        && (map.is_wall(idx - 1) || map.is_door(idx - 1))
        && (map.is_wall(idx + 1) || map.is_door(idx + 1))
        && !map.is_door(idx - map.width as usize)
        && !map.is_door(idx - map.width as usize - 1)
        && !map.is_door(idx + map.width as usize)
        && !map.is_door(idx + map.width as usize + 1)
    {
        return true;
    }

    // East/West
    if map.is_floor(idx)
        && (map.is_wall(idx - map.width as usize) || map.is_door(idx - map.width as usize))
        && (map.is_wall(idx + map.width as usize) || map.is_door(idx + map.width as usize))
        && !map.is_door(idx - 1)
        && !map.is_door(idx - 2)
        && !map.is_door(idx + 1)
        && !map.is_door(idx + 2)
    {
        return true;
    }

    false
}
