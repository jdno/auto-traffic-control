use dashmap::DashMap;

use atc::v1::Airplane;

pub use self::watcher::StoreWatcher;

mod watcher;

pub type Store = DashMap<String, Airplane>;
