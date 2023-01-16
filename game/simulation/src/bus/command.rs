use std::fmt::{Display, Formatter};

use crate::component::{AirplaneId, FlightPlan};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Command {
    StartGame,
    UpdateFlightPlan(AirplaneId, FlightPlan),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Command::StartGame => "StartGame",
            Command::UpdateFlightPlan(_, _) => "UpdateFlightPlan",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let command = Command::StartGame;
        assert_eq!("StartGame", command.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Command>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Command>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Command>();
    }
}
