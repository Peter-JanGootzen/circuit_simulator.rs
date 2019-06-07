use super::node::NodeTrait;

pub struct Circuit {
    pub nodes: Vec<Box<dyn NodeTrait>>;
    pub last_node: Box<dyn NodeTrait>;
}

impl NodeTrait for Circuit {
    fn get_output(&self) -> Output {
        self.last_node.get_output();
    }
}
