use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Running {}

impl Running {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for Running {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "running")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let running = Running::new();
        assert_eq!("running", running.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Running>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Running>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Running>();
    }
}
