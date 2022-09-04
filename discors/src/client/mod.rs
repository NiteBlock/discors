use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};

///! todo!
use self::{
    intents::Intents,
    state::{Build, Starting, State},
};
use crate::error::Error;

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

    pub async fn start(self) -> Result<Client<Starting>, Error> {
        Ok(Client::<Starting>::new(self.state.start()?).await?)
    }
}

impl Client<Starting> {
    pub async fn new(state: Starting) -> Result<Self, Error> {
        let builder = ClientBuilder::new().default_headers({
            let mut x = HeaderMap::new();
            x.insert(
                "Authorization",
                HeaderValue::from_str(&state.token)
                    .map_err(|_errr| Error::ConfigurationError("Invalid token passed."))?,
            );
            x
        });
        Ok(Self { state })
    }
}
