use futures::StreamExt;
use futures_async_stream::try_stream;

use super::builder::Builder;
use super::SubProcess;
use crate::exchange::message::Message;
use crate::process::error::{ProcessError, Result};
use crate::process::BoxedSubProcess;

pub struct MockBuilder;

impl Builder for MockBuilder {
    fn build(self, mut parent: Vec<BoxedSubProcess>) -> Result<BoxedSubProcess> {
        if parent.len() != 1 {
            return Err(ProcessError::Create(
                "MockBuilder should have one child".to_string(),
            ));
        }
        Ok(Box::new(Mock {
            parent: parent.pop().unwrap(),
        }))
    }
}

/// `MockProcess` is a mock subprocess for test.
struct Mock {
    parent: BoxedSubProcess,
}

impl Mock {
    #[try_stream(ok = Message, error = ProcessError)]
    async fn execute_inner(self: Box<Self>) {
        #[for_await]
        for msg in self.parent.execute() {
            yield msg?;
        }
    }
}

impl SubProcess for Mock {
    fn execute(self: Box<Self>) -> super::BoxedMessageStream {
        self.execute_inner().boxed()
    }
}
