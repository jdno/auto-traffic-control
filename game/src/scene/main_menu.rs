use bevy::prelude::*;

use crate::rendering::FONT_COLOR;
use crate::{setup_landscape, AppState};

struct Menu(Entity);

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
    let text = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Ready",
                    TextStyle {
                        font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                        font_size: 48.0,
                        color: FONT_COLOR,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(24.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Send a StartGame request to start a game",
                    TextStyle {
                        font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: FONT_COLOR,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    commands.insert_resource(Menu(text));
}

fn despawn(mut commands: Commands, menu: Res<Menu>) {
    commands.entity(menu.0).despawn_recursive();
}
