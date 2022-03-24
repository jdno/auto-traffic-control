use bevy::prelude::*;

use atc::v1::Tag as ApiTag;

use crate::api::AsApi;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Component)]
pub enum Tag {
    Blue,
    Red,
}

impl AsApi for Tag {
    type ApiType = ApiTag;

    fn as_api(&self) -> Self::ApiType {
        match self {
            Tag::Blue => ApiTag::Blue,
            Tag::Red => ApiTag::Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

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
}
