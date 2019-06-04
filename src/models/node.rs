extern crate objekt;
use super::output::Output;

pub trait NodeTrait: objekt::Clone  {
    fn get_output(&self) -> Output;
}
objekt::clone_trait_object!(NodeTrait);

pub type Nodes = Vec<Box<dyn NodeTrait>>;

#[derive(Clone)]
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

