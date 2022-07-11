# disco-rs
A discord crate for rust!

## Rationale 
All current discord crates require an often very obscured syntax and aren't oriented directly around the end-developer and begginer expeirence when developing bots in them. This crate should allow for basic clients to run in a simple manner, but with scalability and control for the developer.

### Simple Example

```rs
// /src/main.rs
use disco_rs::Client;


#[tokio::main]
fn main() {
    let mut client = Client::new();
    
    client.start("my_token").await;
}
```
