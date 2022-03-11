use dashmap::DashMap;

use atc::v1::Airplane;

pub use self::manager::StoreManager;

mod manager;

pub type Store = DashMap<String, Airplane>;
