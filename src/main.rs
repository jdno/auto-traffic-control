use bevy::prelude::*;

mod map;

/// The width of the game's window
const SCREEN_WIDTH: f32 = 1024.0;

/// The height of the game's window
const SCREEN_HEIGHT: f32 = 768.0;

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
        .run();
}
