use std::sync::Arc;

use bevy::prelude::*;
use tokio::sync::broadcast::channel;

use crate::api::Api;
use crate::command::{Command, CommandReceiver, CommandSender};
use crate::event::{Event, EventReceiver, EventSender};
use crate::scene::{GameOverPlugin, GamePlugin, MainMenuPlugin};
use crate::store::{Store, StoreWatcher};
use crate::systems::*;

mod api;
mod command;
mod components;
mod event;
mod map;
mod rendering;
mod resources;
mod scene;
mod store;
mod systems;

/// The height of the game's window
const SCREEN_HEIGHT: f32 = 640.0;

/// The width of the game's window
const SCREEN_WIDTH: f32 = 800.0;

/// The dimension of a tile
///
/// Tiles must have the same size as the textures that are used to render them. This game uses
/// textures with a size of 32 by 32 pixels, and thus tiles must be 32 pixels high and wide as well.
const TILE_SIZE: i32 = 32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AppState {
    MainMenu,
    Game,
    GameOver,
}

#[tokio::main]
async fn main() {
    let (command_sender, command_receiver) = channel::<Command>(1024);
    let command_sender = CommandSender::new(command_sender);
    let command_receiver = CommandReceiver::new(command_receiver);

    let (event_sender, event_receiver) = channel::<Event>(1024);
    let event_sender = EventSender::new(event_sender);
    let event_receiver = EventReceiver::new(event_receiver);

    let store = Arc::new(Store::new());
    let mut store_watcher = StoreWatcher::new(event_receiver, store.clone());

    let _api_join_handle = tokio::spawn(Api::serve(
        command_sender.clone(),
        event_sender.clone(),
        store,
    ));
    let _drainer_join_handle = tokio::spawn(async move { drain_queue(command_receiver).await });
    let _store_join_handle = tokio::spawn(async move { store_watcher.connect().await });

    App::new()
        // Must be added before the DefaultPlugins
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Auto Traffic Control".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(command_sender)
        .insert_resource(event_sender)
        .add_state(AppState::MainMenu)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(setup_cameras)
        .add_system(change_app_state)
        .run();
}

async fn drain_queue(mut receiver: CommandReceiver) {
    while (receiver.get_mut().recv().await).is_ok() {}
}
