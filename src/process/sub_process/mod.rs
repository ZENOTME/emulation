use super::BoxedMessageStream;

pub mod builder;
pub mod graph;
pub mod mock;
pub mod source;

/// A Process is a chain of `SubProcess` and with a `EndProcess`.
pub trait SubProcess: Send + 'static {
    fn execute(self: Box<Self>) -> BoxedMessageStream;
}
