use super::WINDOW_WIDTH;
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use std::collections::HashMap;

pub struct Popup {
    lines: Vec<String>,
}

impl Popup {
    pub fn new() -> Popup {
        Popup { lines: Vec::new() }
    }

    pub fn add(&mut self, line: String) {
        if self.lines.len() == 1 {
            self.lines
                .push(format!("{}", "-".repeat(self.lines[0].len())));
        }
        let lines = line.lines();
        for l in lines {
            self.lines.push(l.to_string());
        }
    }

    pub fn width(&self) -> i32 {
        let mut max = 0;
        for s in self.lines.iter() {
            if s.len() > max {
                max = s.len();
            }
        }
        max as i32 + 2
    }

    pub fn height(&self) -> i32 {
        self.lines.len() as i32 + 2
    }

    pub fn render_tooltip(&self, x: i32, y: i32, draw_batch: &mut DrawBatch) {
        let white = color("BrightWhite", 1.0);
        let gray = color("BrightBlack", 1.0);
        let black = color("Background", 1.0);
        draw_batch.draw_box(
            Rect::with_size(x, y, self.width() - 1, self.height() - 1),
            ColorPair::new(gray, black),
        );
        draw_batch.fill_region(
            Rect::with_size(x + 1, y + 1, self.width() - 3, self.height() - 3),
            ColorPair::new(black, black),
            ' ' as u16,
        );
        for (i, s) in self.lines.iter().enumerate() {
            let fg = if i < 2 { white } else { gray };
            draw_batch.print_color(
                Point::new(x + 1, y + i as i32 + 1),
                &s,
                ColorPair::new(fg, black),
            );
        }
    }

    pub fn render_popup(&self, draw_batch: &mut DrawBatch) {
        let white = color("BrightWhite", 1.0);
        let gray = color("BrightBlack", 1.0);
        let black = color("Background", 1.0);
        let x = (WINDOW_WIDTH - 1 - self.width()).abs();
        let y = 1;
        draw_batch.draw_box(
            Rect::with_size(x, y, self.width() - 1, self.height() - 1),
            ColorPair::new(gray, black),
        );
        draw_batch.fill_region(
            Rect::with_size(x + 1, y + 1, self.width() - 3, self.height() - 3),
            ColorPair::new(black, black),
            ' ' as u16,
        );
        for (i, s) in self.lines.iter().enumerate() {
            let fg = if i < 2 { white } else { gray };
            draw_batch.print_color(
                Point::new(x + 1, y + i as i32 + 1),
                &s,
                ColorPair::new(fg, black),
            );
        }
    }
}

pub fn draw_list_items(
    items: &HashMap<String, u32>,
    items_vec: &Vec<String>,
    x1: i32,
    y1: i32,
    w: i32,
    draw_batch: &mut DrawBatch,
) {
    let black = color("Background", 1.0);
    let white = color("BrightWhite", 1.0);
    let mut i = 0;
    let mut y = y1 + 1;
    for item in items_vec.iter() {
        //for item in items.iter() {
        draw_batch.set(
            Point::new(x1 + 1, y),
            ColorPair::new(white, black),
            97 + i as FontCharType,
        );
        draw_batch.print_color(
            Point::new(x1 + 2, y),
            //format!(") {}", &item.0),
            format!(") {}", item),
            ColorPair::new(white, black),
        );
        //let x2 = x1 + (item.0.len() as i32) + 4;
        let x2 = x1 + (item.len() as i32) + 4;
        let ct = (x1 + w) - x2 - 4;
        draw_batch.print_color(
            Point::new(x2, y),
            //format!(" {} x{}", ".".repeat(ct as usize), &item.1),
            format!(
                " {} x{}",
                ".".repeat(ct as usize),
                &items.get(item).unwrap()
            ),
            ColorPair::new(white, black),
        );
        i += 1;
        y += 1;
    }
}

pub fn draw_list(list: Vec<String>, x1: i32, y1: i32, draw_batch: &mut DrawBatch) {
    let black = color("Background", 1.0);
    let white = color("BrightWhite", 1.0);
    let mut i = 0;
    let mut y = y1 + 1;

    for l in list.iter() {
        draw_batch.set(
            Point::new(x1 + 1, y),
            ColorPair::new(white, black),
            97 + i as FontCharType,
        );
        draw_batch.print_color(
            Point::new(x1 + 2, y),
            //format!(") {}", &item.0),
            format!(") {}", l),
            ColorPair::new(white, black),
        );
        i += 1;
        y += 1;
    }
}

pub fn draw_named_box(text: &str, x1: i32, y1: i32, w: i32, h: i32, draw_batch: &mut DrawBatch) {
    let black = color("Background", 1.0);
    let gray = color("BrightBlack", 1.0);

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));
    draw_batch.fill_region(
        Rect::with_size(x1 + 1, y1 + 1, w - 2, h - 2),
        ColorPair::new(black, black),
        ' ' as u16,
    );

    draw_batch.print_color(Point::new(w - 5, y1), text, ColorPair::new(gray, black));
}
