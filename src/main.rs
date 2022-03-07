use bevy::prelude::*;

use crate::systems::*;

mod components;
mod map;
mod systems;

/// The height of the game's window
const SCREEN_HEIGHT: f32 = 768.0;

/// The width of the game's window
const SCREEN_WIDTH: f32 = 1024.0;

/// The dimension of a tile
///
/// Tiles must have the same size as the textures that are used to render them. This game uses
/// textures with a size of 32 by 32 pixels, and thus tiles must be 32 pixels high and wide as well.
const TILE_SIZE: i32 = 32;

fn main() {
    App::new()
        // Must be added before the DefaultPlugins
        .insert_resource(WindowDescriptor {
            title: "Auto Traffic Control".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_airport)
        .add_startup_system(setup_grid)
        .add_startup_system(spawn_airplane)
        .run();
}
