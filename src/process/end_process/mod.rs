use async_trait::async_trait;

use super::error::Result;

/// `EndProcess` represents the last sub process of a process.
#[async_trait]
pub trait EndProcess: Send + 'static {
    async fn execute(self: Box<Self>) -> Result<()>;
}
