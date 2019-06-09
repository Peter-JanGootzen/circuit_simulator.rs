use crate::models::output::Output;
use crate::models::node::NodeTrait;
use crate::models::node::Node;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone)]
pub struct AndGate {
    pub inputs: RefCell<Vec<Rc<RefCell<Node>>>>
}
impl NodeTrait for AndGate {
    fn get_output(&self) -> Output {
        let inputs = self.inputs.borrow();
        if inputs.len() == 2 {
            match inputs[0].borrow().get_output() {
                Output::True(_) => {
                    return match inputs[1].borrow().get_output() {
                        Output::True(delay) => Output::True(delay).add_delay(),
                        Output::False(delay) => Output::False(delay).add_delay()
                    }
                },
                Output::False(delay) => {
                    return Output::False(delay).add_delay()
                }
            }
        } else {
            panic!("AndGate with less or more then 2 inputs tried to calculate it's output");
        }
    }
}