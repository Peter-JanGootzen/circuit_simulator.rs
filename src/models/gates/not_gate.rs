use crate::models::output::Output;
use crate::models::node::NodeTrait;
use crate::models::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct NotGate {
    pub inputs: RefCell<Vec<Rc<RefCell<Node>>>>
}
impl<'a> NodeTrait for NotGate {
    fn get_output(&self) -> Output {
        if self.inputs.borrow().len() != 1 {
            panic!("NotGate with less or more then 1 inputs tried to calculate it's output");
        }
        self.inputs.borrow()[0].borrow().get_output().invert()
    }
}