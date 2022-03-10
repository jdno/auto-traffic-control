#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Event {
    AirplaneDetected,
    AirplaneLanded,
    AirplaneMoved,
    FlightPlanUpdated,
}

#[cfg(test)]
mod tests {
    use super::Event;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Event>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Event>();
    }
}
