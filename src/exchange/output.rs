use std::fmt::Debug;

use async_trait::async_trait;

use super::error::ExchangeResult;
use super::message::Message;

pub type BoxedOutput = Box<dyn Output>;

#[async_trait]
pub trait Output: Debug + Send + Sync + 'static {
    // TODO: Add a identifier for the input.

    // TODO: Maybe message is not appropriate to output driver
    async fn send(&mut self, message: Message) -> ExchangeResult<()>;

    fn boxed(self) -> BoxedOutput
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}
