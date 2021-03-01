use super::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::utils::colors::*;
use bracket_lib::prelude::*;
//use specs::prelude::*;

/*
 *
 * menu.rs
 * -------
 * Yes, 'the' Main Menu.
 * See https://github.com/Bobox214/rs-gliphus/blob/master/src/gui.rs for reference.
 *
 */

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

impl MenuSelection {
    fn get_name(&self) -> String {
        String::from(match self {
            MenuSelection::NewGame => "New Journey",
            MenuSelection::LoadGame => "Continue",
            MenuSelection::Quit => "Abandon",
        })
    }

    fn print(&self, y: i32, selection: MenuSelection, draw_batch: &mut DrawBatch) {
        let mut fg = color("White", 1.0);
        if &selection == self {
            fg = color("Cyan", 0.8);
        }
        draw_batch.print_color_centered(
            y,
            self.get_name(),
            ColorPair::new(fg, color("Background", 1.0)),
        );
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MenuResult {
    NoSelection { selected: MenuSelection },
    Selected { selected: MenuSelection },
}

pub fn main_menu(
    selection: MenuSelection,
    can_continue: bool,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
) -> MenuResult {
    draw_batch.draw_hollow_box(
        Rect::with_size(0, 0, WINDOW_WIDTH - 1, WINDOW_HEIGHT - 1),
        ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
    );

    draw_batch.print_color_centered(
        WINDOW_HEIGHT - 3,
        "IN DEVELOPMENT - 2020",
        ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
    );

    draw_batch.print_color_centered(
        WINDOW_HEIGHT - 6,
        "Press F1 to consult the stars for guidance.",
        ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
    );

    let mut y: i32 = 10;
    // Title
    draw_batch.print_color_centered(
        10,
        "YET ANOTHER NAMELESS ROGUELIKE",
        ColorPair::new(color("Green", 1.0), color("Background", 1.0)),
    );

    let entries = if can_continue {
        vec![
            MenuSelection::LoadGame,
            MenuSelection::NewGame,
            MenuSelection::Quit,
        ]
    } else {
        vec![MenuSelection::NewGame, MenuSelection::Quit]
    };

    y += 15;
    for (i, entry) in entries.iter().enumerate() {
        entry.print(y + i as i32, selection, draw_batch);
        y += 1;
    }

    match term.key {
        None => MenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => MenuResult::NoSelection {
                selected: MenuSelection::Quit,
            },
            VirtualKeyCode::Up | VirtualKeyCode::K => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                MenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::J => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                MenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => MenuResult::Selected {
                selected: selection,
            },
            _ => MenuResult::NoSelection {
                selected: selection,
            },
        },
    }
}
