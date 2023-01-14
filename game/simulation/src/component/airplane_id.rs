use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AirplaneId(String);

impl AirplaneId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Default for AirplaneId {
    fn default() -> Self {
        Self::new("AT-0000".to_string())
    }
}

impl Display for AirplaneId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let id = AirplaneId::new("test".to_string());
        assert_eq!(id.get(), "test");
    }

    #[test]
    fn trait_display() {
        let id = AirplaneId::new("test".to_string());
        assert_eq!(format!("{}", id), "test");
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

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<AirplaneId>();
    }
}
