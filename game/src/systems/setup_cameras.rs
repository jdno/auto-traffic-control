use bevy::prelude::*;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
