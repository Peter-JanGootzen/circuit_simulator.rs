use crate::models::output::Output;
use crate::models::node::NodeTrait;
use crate::models::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct ProbeGate {
    pub inputs: RefCell<Vec<Rc<RefCell<Node>>>>
}
impl NodeTrait for ProbeGate {
    fn get_output(&self) -> Output {
        let inputs = self.inputs.borrow();
        if inputs.len() == 1 {
            return inputs[0].borrow().get_output().add_delay();
        }
        else {
            panic!("ProbeGate tried to calculate it's value with more or less then 1 input");
        }
    }
}