use bevy::prelude::*;

fn main() {
    App::new()
        // Must be added before the DefaultPlugins
        .insert_resource(WindowDescriptor {
            title: "Auto Traffic Control".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .run();
}
