use bevy::prelude::*;

use crate::rendering::FONT_COLOR;
use crate::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(spawn))
            .add_system_set(SystemSet::on_update(AppState::GameOver))
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(despawn));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                        font_size: 48.0,
                        color: FONT_COLOR,
                    },
                ),
                ..Default::default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(24.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Two planes got too close to each other. The simulation was aborted.",
                    TextStyle {
                        font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: FONT_COLOR,
                    },
                ),
                ..Default::default()
            });
        });
}

fn despawn(mut commands: Commands, query: Query<Entity, Without<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
