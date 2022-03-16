use bevy::prelude::*;

pub const AIRPLANE_SIZE: f32 = 24.0;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct Airplane;

#[cfg(test)]
mod tests {
    use super::Airplane;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Airplane>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Airplane>();
    }
}
