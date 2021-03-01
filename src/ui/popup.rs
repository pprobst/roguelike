use super::common::Popup;
use bracket_lib::prelude::*;

/*
 *
 * popup.rs
 * -----------
 * Display a popups when an important message needs to be displayed to the player.
 *
 */

pub fn show_context_dir(draw_batch: &mut DrawBatch) {
    let mut popup = Popup::new();
    popup.add("Which direction?".to_string());
    popup.add(format!("Press a movement key\nto indicate direction."));
    popup.render_popup(draw_batch);
}
