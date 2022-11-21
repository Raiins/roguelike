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

// Our state has a world
pub struct State {
    ecs: World,
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

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        // Get the positions of entities and what to render
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();


        // Render that shit
        for (pos, render) in (&positions, &renderables).join() { 
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// What systems does the state have, and allows shit to run and modify stuff
impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
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

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

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
        .build();
    
    // runs the main loop from the rltk library 
    rltk::main_loop(context, gs)
}