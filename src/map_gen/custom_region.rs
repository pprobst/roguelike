use super::common::{circular_region, rect_region};
use crate::components::Position;
use bracket_lib::prelude::DistanceAlg;

#[derive(Clone, Debug, PartialEq)]
pub struct CustomRegion {
    pub pos: Vec<Position>,
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub circular: bool,
}

#[allow(dead_code)]
impl CustomRegion {
    pub fn new_rect(x1: i32, y1: i32, width: i32, height: i32) -> Self {
        let s = width * height;
        Self {
            pos: rect_region(x1, y1, width, height),
            x1,
            x2: x1 + width,
            y1,
            y2: y1 + height,
            width,
            height,
            size: s,
            circular: false,
        }
    }

    pub fn new_circ(x1: i32, y1: i32, radius: i32) -> Self {
        let region = circular_region(x1, y1, radius);
        let w = radius * 2;
        let h = radius * 2;
        let s = w * h;
        Self {
            pos: region,
            x1,
            x2: x1 + w,
            y1,
            y2: y1 + h,
            width: w,
            height: h,
            size: s,
            circular: true,
        }
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        if !self.circular {
            return p.x > self.x1 && p.x < self.x2 && p.y > self.y1 && p.y < self.y2;
        }
        self.in_bounds_circle(p)
    }

    pub fn in_bounds_circle(&self, p: Position) -> bool {
        let c = self.get_center();
        let d = DistanceAlg::Pythagoras.distance2d(c, p);
        d < ((self.x2 - self.x1) / 2) as f32
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.pos.to_vec()
    }

    pub fn get_center(&self) -> Position {
        Position::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
