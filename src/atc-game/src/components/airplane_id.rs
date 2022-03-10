use bevy::prelude::*;

use crate::api::IntoApi;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct AirplaneId(String);

impl AirplaneId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl IntoApi for AirplaneId {
    type ApiType = String;

    fn into_api(self) -> Self::ApiType {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::api::IntoApi;

    use super::AirplaneId;

    #[test]
    fn get() {
        let id = "test";
        let airplane_id = AirplaneId::new(String::from(id));

        assert_eq!(id, airplane_id.get());
    }

    #[test]
    fn trait_into_api() {
        let id = String::from("test");
        let airplane_id = AirplaneId::new(id.clone());

        assert_eq!(id, airplane_id.into_api());
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
