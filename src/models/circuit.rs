use super::node::NodeTrait;
use super::output::Output;

#[derive(Clone)]
pub struct Circuit {
    pub nodes: Vec<Box<dyn NodeTrait>>,
    pub last_node: Box<dyn NodeTrait>
}

impl NodeTrait for Circuit {
    fn get_output(&self) -> Output {
        return self.last_node.get_output();
    }
    fn get_inputs(&self) -> &Vec<Box<dyn NodeTrait>> {
        return self.last_node.get_inputs();
    }
}
