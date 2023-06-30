use super::error::ExchangeResult;

#[derive(Debug, Clone)]
pub struct Message {
    // TODO: Complete the message.
    pub value: String,
}
pub type ExchangeMessage = ExchangeResult<Message>;

pub trait MessageStream = futures::Stream<Item = ExchangeMessage> + Send;
