use futures::stream::select_all;
use futures::StreamExt;
use futures_async_stream::try_stream;

use super::builder::Builder;
use crate::exchange::input::BoxedInput;
use crate::exchange::message::Message;
use crate::process::error::{ProcessError, Result};
use crate::process::BoxedSubProcess;

pub struct SourceBuilder {
    inputs: Vec<BoxedInput>,
}

impl SourceBuilder {
    pub fn new(inputs: Vec<BoxedInput>) -> Self {
        Self { inputs }
    }
}

impl Builder for SourceBuilder {
    fn build(self, parent: Vec<BoxedSubProcess>) -> Result<BoxedSubProcess> {
        if !parent.is_empty() {
            return Err(ProcessError::Create(
                "SourceBuilder should have no child".to_string(),
            ));
        }
        Ok(Box::new(Source::new(self.inputs)))
    }
}

// `Source` is a subprocess read inputs.
pub struct Source {
    inputs: Vec<BoxedInput>,
}

impl Source {
    pub fn new(inputs: Vec<BoxedInput>) -> Self {
        Self { inputs }
    }

    #[try_stream(ok = Message, error = ProcessError)]
    async fn execute_inner(self: Box<Self>) {
        #[for_await]
        for msg in select_all(self.inputs) {
            yield msg?;
        }
    }
}

impl super::SubProcess for Source {
    fn execute(self: Box<Self>) -> super::BoxedMessageStream {
        self.execute_inner().boxed()
    }
}
