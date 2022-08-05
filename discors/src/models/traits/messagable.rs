use async_trait::async_trait;

#[async_trait]
pub trait Messagable {
    async fn send();
}
