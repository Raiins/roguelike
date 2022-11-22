use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

// A component that holds a 2D position of the entity
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// A component that just means that the entity can be rendered to the screen
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

// Player component
#[derive(Component, Debug)]
pub struct Player { }

// Viewshed makes parts of the map invisible
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

// Monster Component
#[derive(Component, Debug)]
pub struct Monster { }

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}