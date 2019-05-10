use super::output::Output;

pub trait NodeTrait {
    fn get_output(&self) -> Output;
}

pub struct NodeStruct<Gate> {
    pub inputs: Vec<Box<dyn NodeTrait>>,
    pub gate: Gate
}
impl<Gate> NodeStruct<Gate> {
    pub fn new(gate: Gate) -> NodeStruct<Gate> {
        NodeStruct {
            inputs: Vec::new(),
            gate: gate
        }
    }
}

pub type Nodes = Vec<Box<dyn NodeTrait>>;
