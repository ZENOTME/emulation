use futures_async_stream::for_await;

use super::error::ProcessResult;
use super::{BoxedSubProcess, EndProcess};
use crate::exchange::output::BoxedOutput;

// `Broadcast` is a sub process broadcast to output
pub struct Broadcast {
    outputs: Vec<BoxedOutput>,
    input: BoxedSubProcess,
}

impl Broadcast {
    pub fn new(outputs: Vec<BoxedOutput>, input: BoxedSubProcess) -> Self {
        Self { outputs, input }
    }
}

#[async_trait::async_trait]
impl EndProcess for Broadcast {
    async fn execute(mut self: Box<Self>) -> ProcessResult<()> {
        #[for_await]
        for msg in self.input.execute() {
            let msg = msg?;
            for output in &mut self.outputs {
                output.send(msg.clone()).await.unwrap();
            }
        }
        Ok(())
    }
}
