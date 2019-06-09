use super::output::Output;
use super::node::NodeTrait;
use super::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct NotGate {
    pub inputs: RefCell<Vec<Rc<Node>>>
}
#[derive(Clone)]
pub struct OrGate {
    pub inputs: RefCell<Vec<Rc<Node>>>
}
#[derive(Clone)]
pub struct AndGate {
    pub inputs: RefCell<Vec<Rc<Node>>>
}
#[derive(Clone)]
pub struct ProbeGate {
    pub inputs: RefCell<Vec<Rc<Node>>>
}
#[derive(Clone)]
pub struct SignalGate {
    pub signal: Output
}

impl<'a> NodeTrait for NotGate {
    fn get_output(&self) -> Output {
        if self.inputs.borrow().len() == 0 {
            println!("Not gate with 0 inputs!");
            return Output::False
        }
        self.inputs.borrow()[0].get_output().invert()
    }
}

impl NodeTrait for SignalGate {
    fn get_output(&self) -> Output {
        self.signal.clone()
    }
}

impl NodeTrait for AndGate {
    fn get_output(&self) -> Output {
        let inputs = self.inputs.borrow();
        if inputs.len() == 2 {
            match inputs[0].get_output() {
                Output::True => {
                    return match inputs[1].get_output() {
                        Output::True => Output::True,
                        Output::False => Output::False
                    }
                },
                Output::False => {
                    return Output::False
                }
            }
        } else {
            return Output::False
        }
    }
}

impl NodeTrait for OrGate {
    fn get_output(&self) -> Output {
        let inputs = self.inputs.borrow();
        if inputs.len() >= 1 {
            match inputs[0].get_output() {
                Output::True => {
                    return Output::True;
                },
                Output::False => {
                    return match inputs[1].get_output() {
                        Output::True =>  Output::True,
                        Output::False => Output::False
                    }
                }
            }
        } else {
            return Output::False;
        }
    }
}

impl NodeTrait for ProbeGate {
    fn get_output(&self) -> Output {
        return self.inputs.borrow()[0].get_output();
    }
}
