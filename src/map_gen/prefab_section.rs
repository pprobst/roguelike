use super::{Map, Point};

/*
 *
 * prefab_section.rs
 * -----------------
 * Inserts pre-made structures on a map.
 * Based on the examples by TheBracket.
 *
 */

pub struct PrefabSection {
    template: &'static str,
    width: i32,
    height: i32,
    pos: Point,
}

#[allow(dead_code)]
impl PrefabSection {
    pub fn generate(&mut self, pt: Point, map: &mut Map) {
        self.pos = pt;
        let v = self.str_to_vec();
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if map.in_map_bounds_xy(x + pt.x, y + pt.y) {
                    let idx = map.idx(x + pt.x, y + pt.y);
                    map.paint_tile_char(idx, v[i])
                }
                i += 1;
            }
        }
    }

    fn str_to_vec(&self) -> Vec<char> {
        let mut string_vec: Vec<char> = self
            .template
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        for c in string_vec.iter_mut() {
            if *c as u8 == 160u8 {
                *c = ' ';
            }
        }
        string_vec
    }
}

#[allow(dead_code)]
pub const HOUSE01: PrefabSection = PrefabSection {
    template: HOUSE01STR,
    width: 16,
    height: 7,
    pos: Point { x: 0, y: 0 }, //pos: Point::new(0, 0),
};

const HOUSE01STR: &str = "
##########.⌠.⌠..
#________####..⌠
#___________###.
#_____________+.
#___________###.
#________####.⌠.
##########.⌠.⌠.⌠
";
