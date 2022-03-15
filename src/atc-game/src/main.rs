use std::sync::Arc;

use bevy::prelude::*;
use parking_lot::Mutex;
use tokio::sync::broadcast::{channel, Receiver};

use atc::v1::get_game_state_response::GameState;

use crate::api::Api;
use crate::command::Command;
use crate::event::{Event, EventBus};
use crate::state::{GameStateReadyPlugin, GameStateRunningPlugin, GameStateWatcher};
use crate::store::{Store, StoreWatcher};
use crate::systems::*;

mod api;
mod command;
mod components;
mod event;
mod map;
mod state;
mod store;
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

type SharedGameState = Arc<Mutex<GameState>>;

#[tokio::main]
async fn main() {
    let (command_sender, command_receiver) = channel::<Command>(1024);
    let (event_sender, event_receiver) = channel::<Event>(1024);

    let game_state = Arc::new(Mutex::new(GameState::Ready));
    let mut game_state_watcher =
        GameStateWatcher::new(event_sender.subscribe(), game_state.clone());

    let store = Arc::new(Store::new());
    let mut store_watcher = StoreWatcher::new(event_receiver, store.clone());

    let _api_join_handle = tokio::spawn(Api::serve(
        command_sender.clone(),
        event_sender.clone(),
        game_state,
        store,
    ));
    let _drainer_join_handle = tokio::spawn(async move { drain_queue(command_receiver).await });
    let _game_state_join_handle = tokio::spawn(async move { game_state_watcher.connect().await });
    let _store_join_handle = tokio::spawn(async move { store_watcher.connect().await });

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
        .insert_resource(command_sender)
        .insert_resource(event_sender)
        .add_state(GameState::Ready)
        .add_plugin(GameStateReadyPlugin)
        .add_plugin(GameStateRunningPlugin)
        .add_startup_system(setup_cameras)
        .add_system(change_app_state)
        .run();
}

async fn drain_queue<T>(mut receiver: Receiver<T>)
where
    T: Clone,
{
    while (receiver.recv().await).is_ok() {}
}
