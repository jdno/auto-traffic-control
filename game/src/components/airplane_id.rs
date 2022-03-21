use bevy::prelude::*;

use crate::api::AsApi;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct AirplaneId(String);

impl AirplaneId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    #[allow(dead_code)] // TODO: Remove when the id is read
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl AsApi for AirplaneId {
    type ApiType = String;

    fn as_api(&self) -> Self::ApiType {
        self.0.clone()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AirplaneIdGenerator {
    last_id: u32,
}

impl AirplaneIdGenerator {
    pub fn generate(&mut self) -> AirplaneId {
        self.last_id += 1;

        AirplaneId(format!("AT-{:0width$}", self.last_id, width = 4))
    }
}

#[cfg(test)]
mod tests {
    use crate::api::AsApi;

    use super::{AirplaneId, AirplaneIdGenerator};

    #[test]
    fn get() {
        let id = "test";
        let airplane_id = AirplaneId(String::from(id));

        assert_eq!(id, airplane_id.get());
    }

    #[test]
    fn generate() {
        let mut generator = AirplaneIdGenerator::default();

        assert_eq!("AT-0001", generator.generate().get());
        assert_eq!("AT-0002", generator.generate().get());
    }

    #[test]
    fn trait_as_api() {
        let id = String::from("test");
        let airplane_id = AirplaneId(id.clone());

        assert_eq!(id, airplane_id.as_api());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AirplaneId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AirplaneId>();
    }
}
