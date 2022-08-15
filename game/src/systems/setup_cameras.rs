use bevy::prelude::*;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
