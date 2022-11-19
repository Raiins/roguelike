use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

// Player component
#[derive(Component, Debug)]
struct Player {}

// Try to update the player positions
fn try_move_player (delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        // If nothing gets pressed
        None => {}
        // 
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
// A component that holds a 2D position of the entity
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

// A component that just means that the entity can be rendered to the screen
#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

// A component that moves things to the left
#[derive(Component)]
struct LeftMover {} 

struct LeftWalker {} 

 
// <'a> tags are lifetime specifiers in this case they must exist long enough for our system to run
impl<'a> System <'a> for LeftWalker {

    // This line tells specs what the system needs to access in this case in needs to update the pos and needs to read leftmover
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, Position>);

    // What happens when it runs
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

// Our state has a world
struct State {
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
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
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
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // creating our main character
    gs.ecs
        .create_entity()
        .with(Position {
            x: 40, 
            y: 25
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    
    // Create 10 smily faces
    for i in 0..10 {
        gs.ecs
        .create_entity()
        .with(Position {
            x: i * 7,
            y: 20,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(LeftMover {})
        .build();
    }

    // runs the main loop from the rltk library 
    rltk::main_loop(context, gs)
}