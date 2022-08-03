// Client<S> is a generic type parameterized by a state type S.
// The state type S is a type that implements the State trait.

use super::intents::Intents;
use std::fmt::Debug;

// check out this cool hack I stole
// https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/core/lib/src/phase.rs#L8-L29
mod private {

    pub trait PrivatizedState {}
}

pub trait State: private::PrivatizedState + Debug {}

macro_rules! state {
    ($state:ident => { $($fields:tt)* }) => {
        #[derive(Debug)]
        pub struct $state {
            $($fields)*
        }
        impl private::PrivatizedState for $state {}
        impl State for $state {}
    };
}

state! { Build => {
    pub token: Option<String>,
    pub intents: Intents,
}}

impl Default for Build {
    fn default() -> Self {
        Build {
            token: None,
            intents: Intents::EMPTY,
        }
    }
}

impl Build {
    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }
    /// Adds the intents (BitOr with current)
    pub fn intent(&mut self, intent: Intents) -> &mut Self {
        self.intents |= intent;
        self
    }
    /// Sets the intents
    pub fn intents(&mut self, intents: Intents) -> &mut Self {
        self.intents = intents;
        self
    }
}
