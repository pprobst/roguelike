use super::{
    components::*,
    input::*,
    killer::remove_dead_entities,
    log::*,
    map_gen::*,
    raws::*,
    renderer::{reload_colors, render_all},
    spawner::{create_player, equip_player},
    systems::*,
    ui::menu::MenuSelection,
    SHOW_MAP,
};
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;

/*
 *
 * state.rs
 * --------
 * Controls the running systems, game states and other main functions at every tick.
 *
 */

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
    PlayerTurn,
    MobTurn,
    Targeting,
    ChooseActionDir,
    Inventory,
    Equipment,
    ItemUse,
    AccessContainer,
    Mapgen,
    Menu { menu_selection: MenuSelection },
    NextLevel,
}

pub struct State {
    pub ecs: World,
    pub resources: Resources,
    pub systems: Schedule,
    pub runstate: RunState,
    pub show_map: bool,
    pub in_menu: bool,
    pub map_generator: MapGenerator,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map = Map::new(80, 60, TileType::Floor, None);
        let mut player = create_player(&mut ecs);
        let mut log = Log::new();
        resources.insert(rng);
        resources.insert(map);
        resources.insert(player);
        resources.insert(RunState::Start);
        resources.insert(log);
        equip_player(&mut ecs);
        Self {
            ecs,
            resources,
            systems: build_systems_scheduler(),
            runstate: RunState::Start,
            show_map: SHOW_MAP,
            in_menu: true,
            map_generator: MapGenerator::new(),
        }
    }

    fn run_systems(&mut self) {
        // TODO
        /*
        let mut vis = FOVSystem {};
        vis.run_now(&self.ecs);
        let mut hostile_ai = HostileAISystem {};
        hostile_ai.run_now(&self.ecs);
        let mut mapping = MappingSystem {};
        mapping.run_now(&self.ecs);
        let mut reload = WeaponReloadSystem {};
        reload.run_now(&self.ecs);
        let mut melee = MeleeSystem {};
        melee.run_now(&self.ecs);
        let mut missile = MissileSystem {};
        missile.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut collect_item = ItemCollectSystem {};
        collect_item.run_now(&self.ecs);
        let mut drop_item = ItemDropSystem {};
        drop_item.run_now(&self.ecs);
        let mut consumable = ConsumableSystem {};
        consumable.run_now(&self.ecs);
        let mut equip = EquipmentSystem {};
        equip.run_now(&self.ecs);
        self.ecs.maintain();
        */
    }

    /*
    // TODO
    fn run_collect_system(&mut self) {
        let mut collect_item = ItemCollectSystem {};
        collect_item.run_now(&self.ecs);
    }
    */

    pub fn generate_new_map(&mut self, width: i32, height: i32) -> Map {
        self.map_generator.push_map(width, height);
        let idx = self.map_generator.get_last_map_idx();
        self.map_generator.gen_map(idx);
        self.set_curr_map(idx);
        self.map_generator.get_map(idx)
    }

    pub fn entities_to_delete(&mut self) -> Vec<Entity> {
        <Entity>::query()
            .iter(&self.ecs)
            .filter(|entity| {
                let entry = self.ecs.entry(**entity).unwrap();
                // Don't delete the player
                if let Ok(_player) = entry.get_component::<Player>() {
                    //entities.push(entity);
                    return false;
                }

                // Don't delete the player's equipment
                if let Ok(_inv) = entry.get_component::<Inventory>() {
                    return false;
                }
                if let Ok(_equip) = entry.get_component::<super::components::Equipment>() {
                    return false;
                }
                true
            })
            .cloned()
            .collect()
    }

    pub fn populate_map(&mut self) {
        let idx = self.map_generator.get_last_map_idx();
        self.map_generator.spawn_entities(&mut self.ecs, idx);
    }

    pub fn set_curr_map(&mut self, idx: usize) {
        let mut curr_map = self.resources.get_mut::<Map>().unwrap();
        *curr_map = self.map_generator.get_map(idx);
    }

    pub fn set_colorscheme(&mut self, colorscheme: &str, term: &mut BTerm, runstate: RunState) {
        &COLORS.lock().unwrap().set_curr_colorscheme(colorscheme);
        reload_colors(&mut self.ecs, &mut self.resources, term, runstate);
    }
}

impl GameState for State {
    fn tick(&mut self, term: &mut BTerm) {
        let mut curr_state;
        {
            let runstate = self.resources.get::<RunState>().unwrap();
            curr_state = *runstate;
        }

        // State machine.
        match curr_state {
            RunState::Menu { .. } => {}
            RunState::Start => {
                self.in_menu = false;
                if self.show_map {
                    curr_state = RunState::Mapgen;
                } else {
                    curr_state = RunState::Running;
                }
            }
            RunState::Running => {
                self.run_systems();
                curr_state = RunState::Waiting;
            }
            RunState::Waiting => {
                curr_state = player_input(self, term);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                curr_state = RunState::MobTurn;
            }
            RunState::MobTurn => {
                self.run_systems();
                curr_state = RunState::Waiting;
            }
            RunState::Targeting => {
                curr_state = targeting_input(self, term);
            }
            RunState::ChooseActionDir => {
                curr_state = action_dir_input(self, term);
            }
            RunState::Inventory => {
                curr_state = RunState::Inventory;
                // Will change state on rendering (messy, but sometimes we just need things to work).
            }
            RunState::Equipment => {
                curr_state = RunState::Equipment;
            }
            RunState::ItemUse => {
                curr_state = RunState::ItemUse;
            }
            RunState::AccessContainer => {
                //self.run_collect_system();
                curr_state = RunState::AccessContainer;
            }
            RunState::Mapgen => match term.key {
                None => {
                    //self.run_systems();
                }
                Some(key) => {
                    if let VirtualKeyCode::Space = key {
                        let mut cb = CommandBuffer::new(&mut self.ecs);
                        for ent in self.entities_to_delete() {
                            cb.remove(ent);
                        }
                        self.generate_new_map(80, 60);
                        self.populate_map();
                    }
                    if let VirtualKeyCode::Return = key {
                        self.show_map = false;
                        curr_state = RunState::Running;
                    }
                }
            },
            RunState::NextLevel => {
                let mut cb = CommandBuffer::new(&mut self.ecs);
                for ent in self.entities_to_delete() {
                    for ent in self.entities_to_delete() {
                        cb.remove(ent);
                    }
                }
                self.generate_new_map(80, 60);
                self.populate_map();
                curr_state = RunState::Running;
            }
        }

        // F3 to enable/disable post-processing effects.
        match term.key {
            None => {}
            Some(key) => {
                if let VirtualKeyCode::F3 = key {
                    term.with_post_scanlines(false);
                }
                if let VirtualKeyCode::F5 = key {
                    self.set_colorscheme("wryan", term, curr_state);
                }
                if let VirtualKeyCode::F6 = key {
                    self.set_colorscheme("vherid_dusk", term, curr_state);
                }
                if let VirtualKeyCode::F7 = key {
                    self.set_colorscheme("spacegray_80s", term, curr_state);
                }
                if let VirtualKeyCode::F8 = key {
                    self.set_colorscheme("tango_dark", term, curr_state);
                }
            }
        }

        {
            let mut write_state = self.resources.get_mut::<RunState>().unwrap();
            *write_state = curr_state;
        }

        remove_dead_entities(&mut self.ecs, &mut self.resources);
        render_all(
            &mut self.ecs,
            &mut self.resources,
            term,
            curr_state,
            self.show_map,
            self.in_menu,
        );
    }
}
