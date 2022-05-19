use bevy::prelude::*;

use crate::event::{Event, EventBus};
use crate::map::Map;
use crate::rendering::FONT_COLOR;
use crate::resources::Score;
use crate::systems::{
    despawn_airplane, detect_collision, follow_flight_plan, generate_flight_plan, land_airplane,
    rotate_airplane, setup_airport, setup_grid, setup_landscape, spawn_airplane,
    update_flight_plan, SpawnTimer,
};
use crate::AppState;

pub struct ScoreOverlay(Entity);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new(Timer::from_seconds(2.0, true)))
            .insert_resource(Map::new())
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(start_game)
                    .with_system(setup_airport)
                    .with_system(setup_grid)
                    .with_system(setup_landscape)
                    .with_system(setup_score),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(follow_flight_plan.label("move"))
                    .with_system(despawn_airplane)
                    .with_system(detect_collision)
                    .with_system(generate_flight_plan)
                    .with_system(land_airplane)
                    .with_system(rotate_airplane)
                    .with_system(spawn_airplane)
                    .with_system(update_flight_plan)
                    .with_system(update_score),
            )
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(end_game));
    }
}

fn start_game(mut commands: Commands, map: Res<Map>, event_bus: Local<EventBus>) {
    commands.insert_resource(Score::new());

    let map = map.into_inner().clone();

    event_bus
        .sender()
        .send(Event::GameStarted(map))
        .expect("failed to send event"); // TODO: Handle error
}

fn end_game(score: Res<Score>, event_bus: Local<EventBus>) {
    event_bus
        .sender()
        .send(Event::GameStopped(*score))
        .expect("failed to send event"); // TODO: Handle error
}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let overlay_id = commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                            font_size: 24.0,
                            color: FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("font/JetBrainsMono-Regular.ttf"),
                            font_size: 24.0,
                            color: FONT_COLOR,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(8.0),
                    left: Val::Px(8.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.insert_resource(ScoreOverlay(overlay_id));
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", score.get());
}
