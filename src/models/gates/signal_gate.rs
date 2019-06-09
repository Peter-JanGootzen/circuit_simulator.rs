use crate::models::output::Output;
use crate::models::node::NodeTrait;
use crate::models::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SignalGate {
    pub signal: Output
}
impl NodeTrait for SignalGate {
    fn get_output(&self) -> Output {
        self.signal.clone()
    }
}