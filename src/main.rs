use rltk::{Rltk, GameState, RGB};
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

// Our state has a world
pub struct State {
    pub ecs: World,
}

// What systems does the state have, and allows shit to run and modify stuff
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

// State is an implementation of GameState from RLTK
impl GameState for State {

    // What happens every frame?
    fn tick(&mut self, ctx : &mut Rltk) {

        // Clear Screen
        ctx.cls();

        // Get the players input
        player_input(self, ctx);

        // Run the logic systems
        self.run_systems();

        draw_map(&self.ecs, ctx);

        // Get the positions of entities and what to render
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();


        // Render that shit
        for (pos, render) in (&positions, &renderables).join() { 
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// Main returns an error for debugging purposes
fn main() -> rltk::BError {
    // Rltk builder creates the window that we display characters on
    use rltk::RltkBuilder;

    // Context is the window 
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Our game state holds a world 
    let mut gs = State{
        ecs: World::new(),
    };

    // gs's world has the components of position and renderable
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);


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
        .build();
    
    // runs the main loop from the rltk library 
    rltk::main_loop(context, gs)
}