use rltk::{Rltk, GameState, RGB, Point, RandomNumberGenerator};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;
mod visiblity_system;
use visiblity_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
    Paused, Running
}

// Our state has a world
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

// What systems does the state have, and allows shit to run and modify stuff
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

// State is an implementation of GameState from RLTK
impl GameState for State {

    // What happens every frame?
    fn tick(&mut self, ctx : &mut Rltk) {

        // Clear Screen
        ctx.cls();

        if self.runstate == RunState::Running {
            // Run the logic systems
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            // Get the players input
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        // Get the positions of entities and what to render
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        // Render that shit
        for (pos, render) in (&positions, &renderables).join() { 
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

// Main returns an error for debugging purposes
fn main() -> rltk::BError {

    let mut rng = RandomNumberGenerator::new();

    // Rltk builder creates the window that we display characters on
    use rltk::RltkBuilder;

    // Context is the window 
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Our game state holds a world 
    let mut gs = State{
        ecs: World::new(),
        runstate: RunState::Running,
    };

    // gs's world has the components of position and renderable
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // creating our main character
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x, 
            y: player_y
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string()
        })
        .build();
    
    for (i,room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1,2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }
        gs.ecs.create_entity()
            .with(Position { 
                x, y})
            .with(Renderable {
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {
                
            })
            .with(Name {
                name: format!("{} #{}", &name, i)
            })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    // runs the main loop from the rltk library 
    rltk::main_loop(context, gs)
}