use std::pin::Pin;

use futures::Stream;

use super::message::{ExchangeMessage, Message, MessageStream};

pub type BoxedInput = Pin<Box<dyn Input>>;

// Provide the interface for the input subprocess.
pub trait Input: MessageStream {
    // TODO: Add a identifier for the input.

    fn boxed_input(self) -> BoxedInput
    where
        Self: Sized + 'static,
    {
        Box::pin(self)
    }
}

/// A mock input to produce a sequence of messages for test. (Number depends on the `target_cnt`.)
pub struct MockInput {
    cnt: usize,
    target_cnt: usize,
}

impl MockInput {
    pub fn new(target_cnt: usize) -> Self {
        Self { cnt: 0, target_cnt }
    }
}

impl Input for MockInput {
    fn boxed_input(self) -> BoxedInput
    where
        Self: Sized + 'static,
    {
        Box::pin(self)
    }
}

impl Stream for MockInput {
    type Item = ExchangeMessage;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.cnt >= self.target_cnt {
            return std::task::Poll::Ready(None);
        }
        self.as_mut().cnt = self.cnt + 1;
        std::task::Poll::Ready(Some(Ok(Message {
            value: format!("MockInput: {}", self.cnt),
        })))
    }
}
