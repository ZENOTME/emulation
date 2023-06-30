use std::collections::HashMap;

use daggy::petgraph::visit::Topo;
use daggy::{Dag, NodeIndex, Walker};

use super::builder::ProcessBuilder;
use crate::process::error::{ProcessError, Result};
use crate::process::BoxedSubProcess;

/// `BuilderNode` used to wrap a builder and make it possible to take the builder out of the node.
struct BuilderNode {
    builder: Option<ProcessBuilder>,
}

impl BuilderNode {
    pub fn new(builder: ProcessBuilder) -> Self {
        Self {
            builder: Some(builder),
        }
    }

    pub fn take_builder(&mut self) -> ProcessBuilder {
        self.builder.take().unwrap()
    }
}

/// `BuilderGraph` is a DAG of SubProcess Builder used to build DAG of SubProcess.
pub struct BuilderGraph {
    dag: Dag<BuilderNode, ()>,
}

impl Default for BuilderGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl BuilderGraph {
    pub fn new() -> Self {
        Self { dag: Dag::new() }
    }

    pub fn add_node(&mut self, builder: ProcessBuilder) -> daggy::NodeIndex {
        self.dag.add_node(BuilderNode::new(builder))
    }

    pub fn add_edge(&mut self, from: daggy::NodeIndex, to: daggy::NodeIndex) -> Result<()> {
        self.dag.add_edge(from, to, ())?;
        Ok(())
    }

    pub fn extend_with_edges(
        &mut self,
        edges: Vec<(daggy::NodeIndex, daggy::NodeIndex)>,
    ) -> Result<()> {
        for (from, to) in edges {
            self.add_edge(from, to)?;
        }
        Ok(())
    }

    pub fn build(mut self) -> Result<BoxedSubProcess> {
        let mut topo = Topo::new(&self.dag);
        let mut subprocess_map = HashMap::<NodeIndex, BoxedSubProcess>::new();

        while let Some(node) = topo.next(&self.dag) {
            let mut parent_subprocesses = vec![];
            let mut parents = self.dag.parents(node);
            while let Some((_, parent_idx)) = parents.walk_next(&self.dag) {
                let parent_subprocess =
                    subprocess_map
                        .remove(&parent_idx)
                        .ok_or(ProcessError::Create(
                            "Parent subprocess not found".to_string(),
                        ))?;
                parent_subprocesses.push(parent_subprocess);
            }
            let builder = self.dag[node].take_builder();
            let subprocess = builder.build(parent_subprocesses)?;
            subprocess_map.insert(node, subprocess);
        }

        if subprocess_map.len() != 1 {
            return Err(ProcessError::Create(
                "Subprocess graph should have only one root subprocess".to_string(),
            ));
        }
        let root_subprocess = subprocess_map
            .into_values()
            .next()
            .ok_or(ProcessError::Create("Empty subprocess graph".to_string()))?;

        Ok(root_subprocess)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use futures_async_stream::for_await;

    use crate::exchange::input::{Input, MockInput};
    use crate::process::sub_process::mock::MockBuilder;
    use crate::process::sub_process::source::SourceBuilder;

    #[tokio::test]
    async fn test_sub_process() -> Result<()> {
        let input = MockInput::new(10);
        let input_2 = MockInput::new(30);

        let mut graph = super::BuilderGraph::new();

        let a = graph
            .add_node(SourceBuilder::new(vec![input.boxed_input(), input_2.boxed_input()]).into());
        let b = graph.add_node(MockBuilder {}.into());
        let c = graph.add_node(MockBuilder {}.into());
        graph.extend_with_edges(vec![(a, b), (b, c)])?;

        let root_subprocess = graph.build()?;

        #[for_await]
        for msg in root_subprocess.execute() {
            println!("{:?}", msg);
        }

        Ok(())
    }
}
