use bevy::prelude::*;

use atc::v1::get_game_state_response::GameState;

struct Menu(Entity);

pub struct GameStateReadyPlugin;

impl Plugin for GameStateReadyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ready).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Ready))
            .add_system_set(SystemSet::on_exit(GameState::Ready).with_system(despawn));
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
                        color: Color::rgb(0.9, 0.9, 0.9),
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
                        color: Color::rgb(0.9, 0.9, 0.9),
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
