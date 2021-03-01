use super::{
    map_gen::Map, raws::*, ui::*, utils::colors::*, Name, Position, Remains, Renderable, RunState,
    Target, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * renderer.rs
 * -----------
 * Controls the rendering of everything on the screen.
 *
 */

pub struct Renderer<'a> {
    pub ecs: &'a World,
    pub term: &'a mut BTerm,
    pub state: RunState,
}

pub fn render_all(ecs: &World, term: &mut BTerm, state: RunState, show_map: bool, in_menu: bool) {
    Renderer { ecs, term, state }.render_all(show_map, in_menu)
}

pub fn reload_colors(ecs: &World, term: &mut BTerm, state: RunState) {
    Renderer { ecs, term, state }.reload_colors()
}

impl<'a> Renderer<'a> {
    /*pub fn new(ecs: &'a World, term: &'a mut BTerm) -> Self {
        Self { ecs, term }
    }*/

    /// Renders all the elements of the game.
    /// * Map;
    /// * Entities;
    /// * UI.
    pub fn render_all(&mut self, show_map: bool, in_menu: bool) {
        let (min_x, max_x, min_y, max_y, x_offset, y_offset) = self.screen_bounds();
        let mut draw_batch = DrawBatch::new();
        let bg = color("Background", 1.0);

        if !show_map {
            self.clear_draw_batch(&mut draw_batch, bg, 1);
            self.render_ui(&mut draw_batch, min_x, min_y);
        }

        self.clear_draw_batch(&mut draw_batch, bg, 0);

        if !in_menu {
            self.render_map(
                &mut draw_batch,
                show_map,
                min_x,
                max_x,
                min_y,
                max_y,
                x_offset,
                y_offset,
            );
            self.render_entitites(&mut draw_batch, show_map, min_x, min_y, x_offset, y_offset);
        }

        draw_batch.submit(0).expect("Batch error.");
        render_draw_buffer(self.term).expect("Render error.");
    }

    fn clear_draw_batch(&self, draw_batch: &mut DrawBatch, bg: RGBA, num: usize) {
        draw_batch.target(num);
        draw_batch.cls_color(bg);
    }

    fn screen_bounds(&mut self) -> (i32, i32, i32, i32, i32, i32) {
        // https://www.reddit.com/r/roguelikedev/comments/8exy6o/brand_new_dev_struggling_with_a_scrolling_screen/
        // Player position.
        let ppos = self.ecs.fetch::<Point>();
        //println!("{}, {}", ppos.x, ppos.y);

        // Size of the map portion shown on screen.
        //let (cam_x, cam_y) = self.term.get_char_size();
        let (cam_x, cam_y) = (WINDOW_WIDTH - X_OFFSET, WINDOW_HEIGHT + Y_OFFSET);

        let min_x = ppos.x - (cam_x / 2) as i32;
        let max_x = min_x + cam_x as i32;
        let min_y = ppos.y - (cam_y / 2) as i32;
        let max_y = min_y + cam_y as i32;
        //println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

        let x_offset = X_OFFSET;
        let y_offset = Y_OFFSET;

        (min_x, max_x, min_y, max_y - y_offset, x_offset, -y_offset)
    }

    /// Renders a targeting path between an origin point and a destiny point.
    fn render_line_path(
        &mut self,
        draw_batch: &mut DrawBatch,
        orig: Point,
        dest: Point,
        render: Renderable,
        covered: bool,
    ) {
        let points = line2d_vector(orig, dest);
        //let points = line2d_bresenham(orig, dest);
        if points.len() > 1 {
            for (i, pt) in points.iter().enumerate() {
                if i == points.len() - 1 {
                    draw_batch.set(
                        *pt,
                        ColorPair::new(render.color.fg, color("BrightBlack", 0.7)),
                        render.glyph,
                    );
                } else if i != 0 {
                    if !covered {
                        draw_batch.set(
                            *pt,
                            ColorPair::new(color("BrightCyan", 1.0), color("Background", 1.0)),
                            to_cp437('∙'),
                        );
                    } else {
                        draw_batch.set(
                            *pt,
                            ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
                            to_cp437('∙'),
                        );
                    }
                }
            }
        }
    }

    fn render_map(
        &mut self,
        draw_batch: &mut DrawBatch,
        show_map: bool,
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
        x_offset: i32,
        y_offset: i32,
    ) {
        let mut map = self.ecs.fetch_mut::<Map>();

        if show_map {
            let _map = map.clone();
            for (idx, tile) in map.tiles.iter_mut().enumerate() {
                let pos = _map.idx_pos(idx);
                draw_batch.set(Point::new(pos.x, pos.y), tile.color, tile.glyph);
            }
            return;
        }

        for (y, y2) in (min_y..max_y).enumerate() {
            for (x, x2) in (min_x..max_x).enumerate() {
                if map.in_map_bounds_xy(x2, y2) {
                    let idx = map.idx(x2, y2);
                    let mut tile = map.tiles[idx];
                    if !tile.visible {
                        tile.shadowed();
                    }
                    if tile.revealed {
                        draw_batch.set(
                            Point::new(x as i32 + x_offset, y as i32 + y_offset),
                            tile.color,
                            tile.glyph,
                        );
                    }
                }
            }
        }
    }

    fn render_entitites(
        &mut self,
        draw_batch: &mut DrawBatch,
        show_map: bool,
        min_x: i32,
        min_y: i32,
        x_offset: i32,
        y_offset: i32,
    ) {
        let map = self.ecs.fetch::<Map>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let targets = self.ecs.read_storage::<Target>();
        let entities = self.ecs.entities();

        let mut render_data = (&positions, &renderables, &entities)
            .join()
            .collect::<Vec<_>>();

        // Sorting renderables by layer: the renderables with layer 0 will be rendered first, that
        // is, bellow the renderables with layer 1 and so on.
        render_data.sort_by(|&a, &b| a.1.layer.cmp(&b.1.layer));

        for (pos, render, ent) in render_data {
            let idx = map.idx(pos.x, pos.y);
            //if map.tiles[idx].visible || show_map {
            if map.tiles[idx].visible {
                let ent_x = pos.x - min_x;
                let ent_y = pos.y - min_y;
                if map.in_map_bounds_xy(ent_x, ent_y) {
                    draw_batch.set(
                        Point::new(ent_x + x_offset, ent_y + y_offset),
                        render.color,
                        render.glyph,
                    );
                    let target = targets.get(ent);
                    if let Some(_target) = target {
                        let cover = _target.covered;
                        let pt = self.ecs.fetch::<Point>();
                        let ppos = *pt;
                        self.render_line_path(
                            draw_batch,
                            Point::new(ppos.x - min_x + x_offset, ppos.y - min_y + y_offset),
                            Point::new(ent_x + x_offset, ent_y + y_offset),
                            *render,
                            cover,
                        );
                    }
                }
            }
        }
    }

    fn render_ui(&mut self, draw_batch: &mut DrawBatch, min_x: i32, min_y: i32) {
        let mut write_state = self.ecs.write_resource::<RunState>();

        match self.state {
            RunState::Menu {
                menu_selection: selection,
            } => {
                let res = menu::main_menu(selection, false, self.term, draw_batch);
                match res {
                    menu::MenuResult::NoSelection { selected } => {
                        *write_state = RunState::Menu {
                            menu_selection: selected,
                        }
                    }
                    menu::MenuResult::Selected { selected } => match selected {
                        menu::MenuSelection::NewGame => {
                            *write_state = RunState::Start;
                        }
                        menu::MenuSelection::LoadGame => {
                            // TODO
                            *write_state = RunState::Start;
                        }
                        menu::MenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            _ => {
                hud::boxes(draw_batch);
                hud::name_stats(self.ecs, draw_batch);
                hud::show_equipped(self.ecs, draw_batch);
                hud::game_log(self.ecs, draw_batch);
                let mouse_pos = self.term.mouse_pos();

                if mouse_pos.0 > X_OFFSET
                    && mouse_pos.0 < WINDOW_WIDTH - 1
                    && mouse_pos.1 < WINDOW_HEIGHT - Y_OFFSET
                    && mouse_pos.1 > 0
                {
                    draw_batch.set_bg(Point::new(mouse_pos.0, mouse_pos.1), color("Cyan", 0.5));
                    tooltips::show_tooltip(self.ecs, self.term, draw_batch, min_x, min_y);
                }

                match self.state {
                    RunState::ChooseActionDir => {
                        popup::show_context_dir(draw_batch);
                    }
                    RunState::Inventory => {
                        let inventory_result =
                            inventory::show_inventory(self.ecs, self.term, draw_batch);
                        if inventory_result == inventory::InventoryResult::Cancel {
                            *write_state = RunState::Running;
                        } else if inventory_result == inventory::InventoryResult::Select {
                            *write_state = RunState::ItemUse;
                        }
                    }
                    RunState::ItemUse => {
                        let inventory_result =
                            inventory::show_use_menu(self.ecs, self.term, draw_batch);
                        if inventory_result == inventory::InventoryResult::Cancel {
                            *write_state = RunState::Running;
                        } else if inventory_result == inventory::InventoryResult::DropItem
                            || inventory_result == inventory::InventoryResult::UseItem
                        {
                            *write_state = RunState::MobTurn;
                        }
                    }
                    RunState::Equipment => {
                        let equip_result =
                            equipment::show_equipment(self.ecs, self.term, draw_batch);
                        if equip_result == equipment::EquipmentResult::Cancel {
                            *write_state = RunState::Running;
                        } else if equip_result == equipment::EquipmentResult::Select {
                            *write_state = RunState::ItemUse;
                        }
                    }
                    RunState::AccessContainer => {
                        let container_result =
                            container::show_container(self.ecs, self.term, draw_batch);
                        if container_result == container::ContainerResult::Cancel {
                            *write_state = RunState::MobTurn;
                        } else if container_result == container::ContainerResult::Select {
                            *write_state = RunState::AccessContainer;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn reload_colors(&mut self) {
        let mut map = self.ecs.fetch_mut::<Map>();

        map.reload_tile_colors();

        let mut renderables = self.ecs.write_storage::<Renderable>();
        let entities = self.ecs.entities();
        let names = self.ecs.read_storage::<Name>();
        let player = self.ecs.fetch::<Entity>();

        for (render, ent, name) in (&mut renderables, &entities, &names).join() {
            if ent == *player {
                render.color = ColorPair::new(color("BrightWhite", 1.0), color("Background", 1.0));
            } else {
                let raws = &RAWS.lock().unwrap();
                let ent_name = &name.name;
                if let Some(renderable) = raws.get_renderable(ent_name) {
                    render.color =
                        ColorPair::new(color(&renderable.fg, 1.0), color(&renderable.bg, 1.0));
                }
            }
        }

        let remains = self.ecs.read_storage::<Remains>();
        for (render, _remain) in (&mut renderables, &remains).join() {
            render.color = ColorPair::new(color("Red", 0.6), color("Background", 1.0));
        }
    }
}
