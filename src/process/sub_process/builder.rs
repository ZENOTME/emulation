use super::mock::MockBuilder;
use super::source::SourceBuilder;
use crate::process::error::Result;
use crate::process::BoxedSubProcess;

pub trait Builder {
    fn build(self, parent: Vec<BoxedSubProcess>) -> Result<BoxedSubProcess>;
}

/// Builder used to build a DAG of `SubProcess`. Every `SubProcess` implement its `Builder`
/// trait.
pub enum ProcessBuilder {
    Source(SourceBuilder),
    Mock(MockBuilder),
}

impl ProcessBuilder {
    pub fn build(self, parent: Vec<BoxedSubProcess>) -> Result<BoxedSubProcess> {
        match self {
            ProcessBuilder::Source(builder) => builder.build(parent),
            ProcessBuilder::Mock(builder) => builder.build(parent),
        }
    }
}

impl From<SourceBuilder> for ProcessBuilder {
    fn from(builder: SourceBuilder) -> Self {
        ProcessBuilder::Source(builder)
    }
}

impl From<MockBuilder> for ProcessBuilder {
    fn from(builder: MockBuilder) -> Self {
        ProcessBuilder::Mock(builder)
    }
}
