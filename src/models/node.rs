use super::output::Output;

pub trait NodeTrait {
    fn get_output(&mut self) -> Output;
}

pub struct NodeStruct<Gate> {
    pub inputs: Vec<Box<dyn NodeTrait>>,
    pub gate: Gate
}

pub type Nodes = Vec<Box<dyn NodeTrait>>;
