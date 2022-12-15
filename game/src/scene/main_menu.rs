use bevy::prelude::*;

use crate::rendering::RenderLayer;
use crate::{setup_landscape, AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_landscape)
                .with_system(spawn),
        )
        .add_system_set(SystemSet::on_update(AppState::MainMenu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, RenderLayer::Ui.z()),
        texture: asset_server.load("sprites/logo.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, -128.0, RenderLayer::Ui.z()),
        texture: asset_server.load("sprites/instructions.png"),
        ..Default::default()
    });
}

fn despawn(mut commands: Commands, query: Query<Entity, Without<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
