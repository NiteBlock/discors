// /src/main.rs
use discors::{channel::Message, event, events, Client, Context};

#[tokio::main]
fn main() {
    // construct a client
    let mut client = Client::new();

    // sets the events for the client
    // the events macro parses on_message to an implementor of the Event trait.
    // This is now a Vec<Box<dyn Event>>
    client.events(events!(on_message));

    // Starts the bot!
    client.start("my_token").await;
}

// This macro will do a lot of things behind the scenes, like figure out what on_message is.
#[event]
fn on_message(ctx: Context, msg: Message) -> discors::Result<()> {
    ctx.reply(format!("Hello, {}", msg.author.name)).await?;
    Ok(())
}
