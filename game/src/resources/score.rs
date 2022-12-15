use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Resource)]
pub struct Score(u32);

impl Score {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Score;

    #[test]
    fn get() {
        let score = Score::new();
        assert_eq!(0, score.get());
    }

    #[test]
    fn increment() {
        let mut score = Score::new();

        score.increment();

        assert_eq!(1, score.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Score>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Score>();
    }
}
