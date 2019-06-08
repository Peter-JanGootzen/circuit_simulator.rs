use super::gates::SignalGate;
use super::node::NodeStruct;
use super::node::NodeTrait;

pub struct Visitor;
pub trait AbstractVisitor {
    fn visit_signal_node(self, node: &NodeStruct<SignalGate>);
}
impl AbstractVisitor for Visitor {
    fn visit_signal_node(self, node: &NodeStruct<SignalGate>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
    }
}

pub trait Visitable {
    fn accept_visitor(&self, v: Visitor);
}
impl<'a> Visitable for NodeStruct<'a, SignalGate> {
    fn accept_visitor(&self, v: Visitor) {
        v.visit_signal_node(&self);
    }
}
