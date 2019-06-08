use super::node::NodeTrait;
use super::output::Output;

#[derive(Clone)]
pub struct Circuit<'a> {
    pub nodes: Vec<&'a Box<dyn NodeTrait>>,
    pub last_node: &'a Box<dyn NodeTrait>
}

impl<'a> NodeTrait for Circuit<'a> {
    fn get_output(&self) -> Output {
        return self.last_node.get_output();
    }
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return self.last_node.get_inputs();
    }
}
