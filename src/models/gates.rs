use super::output::Output;
use super::node::NodeTrait;
use super::node::NodeStruct;
use std::any::Any;

#[derive(Clone)]
pub struct NotGate;
impl<'a> NodeTrait for NodeStruct<'a, NotGate> {
    fn get_output(&self) -> Output {
        self.inputs[0].get_output().invert()
    }
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return &self.inputs;
    }
}

#[derive(Clone)]
pub struct SignalGate {
    pub signal: Output
}
impl<'a> NodeTrait for NodeStruct<'a, SignalGate> {
    fn get_output(&self) -> Output {
        self.gate.signal.clone()
    }
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return &self.inputs;
    }
}

#[derive(Clone)]
pub struct AndGate;
impl<'a> NodeTrait for NodeStruct<'a, AndGate> {
    fn get_output(&self) -> Output {
        if self.inputs.len() == 2 {
            match self.inputs[0].get_output() {
                Output::True => {
                    return match self.inputs[1].get_output() {
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
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return &self.inputs;
    }
}

#[derive(Clone)]
pub struct OrGate;
impl<'a> NodeTrait for NodeStruct<'a, OrGate> {
    fn get_output(&self) -> Output {
        if self.inputs.len() >= 1 {
            match self.inputs[0].get_output() {
                Output::True => {
                    return Output::True;
                },
                Output::False => {
                    return match self.inputs[1].get_output() {
                        Output::True =>  Output::True,
                        Output::False => Output::False
                    }
                }
            }
        } else {
            return Output::False;
        }
    }
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>> {
        return &self.inputs;
    }
}
