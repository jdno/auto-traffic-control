use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Component)]
pub struct Speed(f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Speed;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Speed>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Speed>();
    }
}
