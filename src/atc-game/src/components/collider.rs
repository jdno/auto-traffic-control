use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct Collider;

#[cfg(test)]
mod tests {
    use super::Collider;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Collider>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Collider>();
    }
}
