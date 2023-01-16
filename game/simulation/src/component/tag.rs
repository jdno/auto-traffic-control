use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Tag {
    Blue,
    Red,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Blue => write!(f, "blue"),
            Tag::Red => write!(f, "red"),
        }
    }
}

impl From<Tag> for auto_traffic_control::v1::Tag {
    fn from(tag: Tag) -> Self {
        match tag {
            Tag::Blue => Self::Blue,
            Tag::Red => Self::Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        assert_eq!("blue", Tag::Blue.to_string());
        assert_eq!("red", Tag::Red.to_string());
    }

    #[test]
    fn trait_from_tag() {
        assert_eq!(auto_traffic_control::v1::Tag::Blue, Tag::Blue.into());
        assert_eq!(auto_traffic_control::v1::Tag::Red, Tag::Red.into());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Tag>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Tag>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Tag>();
    }
}
