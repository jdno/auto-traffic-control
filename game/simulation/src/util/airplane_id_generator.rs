use std::fmt::Display;

use crate::component::AirplaneId;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AirplaneIdGenerator {
    last_id: u32,
}

impl AirplaneIdGenerator {
    pub fn generate(&mut self) -> AirplaneId {
        self.last_id += 1;

        AirplaneId::new(format!("AT-{:0width$}", self.last_id, width = 4))
    }
}

impl Display for AirplaneIdGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AirplaneIdGenerator {{ last_id: {} }}", self.last_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut generator = AirplaneIdGenerator::default();

        let id = generator.generate();

        assert_eq!(id.get(), "AT-0001");
    }

    #[test]
    fn trait_display() {
        let generator = AirplaneIdGenerator::default();

        assert_eq!(
            format!("{}", generator),
            "AirplaneIdGenerator { last_id: 0 }"
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AirplaneIdGenerator>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AirplaneIdGenerator>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<AirplaneIdGenerator>();
    }
}
