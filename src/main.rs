#[derive(Debug, Clone)]
enum Output {
    True,
    False
}
impl Output {
    fn invert(&self) -> Output {
        match self {
            Output::True => Output::False,
            Output::False => Output::True
        }
    }
}

trait NodeTrait {
    fn get_output(&mut self) -> Output;
}

struct NodeStruct<Gate> {
    inputs: Vec<Box<dyn NodeTrait>>,
    gate: Gate
}

struct NotGate;
impl NodeTrait for NodeStruct<NotGate> {
    fn get_output(&mut self) -> Output {
        self.inputs[0].get_output().invert()
    }
}

struct SignalGate {
    signal: Output
}
impl NodeTrait for NodeStruct<SignalGate> {
    fn get_output(&mut self) -> Output {
        self.gate.signal.clone()
    }
}

struct AndGate;
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

impl Visitable for NodeStruct<SignalGate> {
    fn accept_visitor(&mut self, v: Visitor) {
        v.visit_signal_gate(&self);
    }
}

type Nodes = Vec<Box<dyn NodeTrait>>;


// Visitor pattern

trait Visitable {
    fn accept_visitor(&mut self, v: Visitor);
}
struct Visitor;
impl Visitor {
    fn visit_signal_gate(self, gate: &NodeStruct<SignalGate>) {
    }
}
// impl visitable
//  v.visit_and_gate(self);
//

fn main() {
    let signal_false_node = NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::False }
    };
    let mut not_node = NodeStruct {
        inputs: vec![Box::new(signal_false_node)],
        gate: NotGate
    };

    let signal_true_node = NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::True }
    };

    let mut and_node = NodeStruct {
        inputs: vec![Box::new(signal_true_node), Box::new(not_node)],
        gate: AndGate
    };

    println!("{:?}", and_node.get_output());
}
