use crate::components::{AirplaneId, FlightPlan};

pub use self::bus::{CommandBus, CommandReceiver, CommandSender};

mod bus;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Command {
    StartGame,
    UpdateFlightPlan(AirplaneId, FlightPlan),
}

#[cfg(test)]
mod test {
    use super::CommandBus;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CommandBus>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CommandBus>();
    }
}
