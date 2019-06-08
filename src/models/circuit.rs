use super::node::NodeTrait;
use super::output::Output;

#[derive(Clone)]
pub struct Circuit {
    pub nodes: Vec<Box<dyn NodeTrait>>,
}

impl NodeTrait for Circuit {
    fn get_output(&self) -> Output {
        return self.get_last_node().get_output();
    }
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return self.get_last_node().get_inputs();
    }
}

impl Circuit {
    pub fn new(nodes: Vec<Box<dyn NodeTrait>>) -> Circuit {
        Circuit {
            nodes: nodes
        }
    }
    pub fn get_last_node(&self) -> &Box<dyn NodeTrait> {
        return &self.nodes[self.nodes.len() - 1];
    }
}
