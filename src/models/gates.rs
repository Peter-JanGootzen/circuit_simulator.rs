use super::output::Output;
use super::node::NodeTrait;
use super::node::NodeStruct;

pub struct NotGate;
impl NodeTrait for NodeStruct<NotGate> {
    fn get_output(&mut self) -> Output {
        self.inputs[0].get_output().invert()
    }
}

pub struct SignalGate {
    pub signal: Output
}
impl NodeTrait for NodeStruct<SignalGate> {
    fn get_output(&mut self) -> Output {
        self.gate.signal.clone()
    }
}

pub struct AndGate;
impl NodeTrait for NodeStruct<AndGate> {
    fn get_output(&mut self) -> Output {
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
}