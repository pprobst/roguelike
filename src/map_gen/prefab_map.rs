use super::{Map, Point, Tile};
use bracket_lib::prelude::{to_char, XpFile};

/*
 *
 * prefab_map.rs
 * -------------
 * Generates a map based on a prefabricated .xp map.
 * Based on the examples by TheBracket.
 *
 */

pub struct PrefabMap {
    template: &'static str,
}

#[allow(dead_code)]
impl PrefabMap {
    pub fn new(template: &'static str) -> Self {
        Self { template }
    }

    pub fn generate(&mut self, map: &mut Map) {
        map.tiles = vec![Tile::floor(); (map.width * map.height) as usize];
        let prefab_map = XpFile::from_resource(self.template).unwrap();

        for layer in &prefab_map.layers {
            println!("height: {}", layer.height);
            println!("width: {}", layer.width);
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    //println!("{}", (cell.ch as u8));
                    //if map.in_map_bounds_xy(x as i32, y as i32) {
                    let idx = map.idx(x as i32, y as i32);
                    //map.paint_tile_char(idx, (cell.ch as u8) as char);
                    map.paint_tile_char(idx, to_char(cell.ch as u8));
                }
            }
        }
    }

    pub fn repeat_template(&mut self, map: &mut Map) {
        let prefab_map = XpFile::from_resource(self.template).unwrap();

        for layer in &prefab_map.layers {
            let tx = map.width / (layer.width as i32);
            let ty = map.height / (layer.height as i32);
            for y in 0..ty {
                for x in 0..tx {
                    let start = Point::new(x * layer.width as i32, y * layer.height as i32);
                    let end =
                        Point::new((x + 1) * layer.width as i32, (y + 1) * layer.height as i32);
                    for py in start.y..end.y {
                        for px in start.x..end.x {
                            let cell = layer
                                .get((px - start.x) as usize, (py - start.y) as usize)
                                .unwrap();
                            let idx = map.idx(px, py);
                            map.paint_tile_char(idx, to_char(cell.ch as u8));
                        }
                    }
                }
            }
        }
    }

    pub fn repeat_template_cont(&mut self, map: &mut Map) {
        let prefab_map = XpFile::from_resource(self.template).unwrap();

        for layer in &prefab_map.layers {
            let xt = layer.width as i32 - 1;
            let yt = layer.height as i32 - 1;
            for y in 0..map.height {
                for x in xt..map.width {
                    let idx = map.idx(xt, y);
                    let tile_to_repeat = map.tiles[idx].ttype;
                    map.paint_tile(map.idx(x, y), tile_to_repeat);
                }
            }
            for y in yt..map.height {
                for x in 0..map.width {
                    let idx = map.idx(x, yt);
                    let tile_to_repeat = map.tiles[idx].ttype;
                    map.paint_tile(map.idx(x, y), tile_to_repeat);
                }
            }
        }
    }
}
