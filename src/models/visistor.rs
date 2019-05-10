impl Visitable for NodeStruct<SignalGate> {
    fn accept_visitor(&mut self, v: Visitor) {
        v.visit_signal_gate(&self);
    }
}



// Visitor pattern

trait Visitable {
    fn accept_visitor(&mut self, v: Visitor);
}
struct Visitor;
impl Visitor {
    fn visit_signal_gate(self, gate: &NodeStruct<SignalGate>) {
    }
}