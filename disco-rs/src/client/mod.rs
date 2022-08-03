///! todo!
use self::{
    intents::Intents,
    state::{Build, State},
};

/// Gateway intents that are used when establishing a client connection with discord.
/// These determine what events the bot recieves and some other ways data works.
///
/// [Read More]()
pub mod intents;
pub mod state;

#[derive(Debug)]
pub struct Client<S>
where
    S: State,
{
    state: S,
}

impl Default for Client<Build> {
    fn default() -> Self {
        Self::new()
    }
}

impl Client<Build> {
    /// Create a Client<Build>, which will allow you to setup options and then create a connection
    pub fn new() -> Self {
        Client {
            state: Build::default(),
        }
    }
    /// Sets the token for the client's connection.
    pub fn token(&mut self, token: String) -> &mut Self {
        self.state.token(token);
        self
    }

    pub fn intent(&mut self, intent: Intents) -> &mut Self {
        self.state.intent(intent);
        self
    }

    pub fn intents(&mut self, intents: Intents) -> &mut Self {
        self.state.intents(intents);
        self
    }
}
