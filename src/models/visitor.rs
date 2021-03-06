use crate::models::node::Node;
use crate::models::node::NodeTrait;
use crate::models::output::Output;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::borrow::Borrow;

pub struct ConcreteVisitor {
    received_string: String
}
impl ConcreteVisitor {
    pub fn new() -> Box<dyn VisitorTrait> {
        let cv = Box::new(ConcreteVisitor {
            received_string: String::new()
        });
        cv
    }
}
pub trait VisitorTrait {
    fn visit_signal_node(&mut self, node: &Node);
    fn visit_not_node(&mut self, node: &Node);
    fn visit_and_node(&mut self, node: &Node);
    fn visit_or_node(&mut self, node: &Node);
    fn visit_probe_node(&mut self, node: &Node);
    fn get_output(&self) -> String;
}
impl VisitorTrait for ConcreteVisitor {
    fn get_output(&self) -> String {
        self.received_string.clone()
    }
    fn visit_signal_node(&mut self, node: &Node) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a SignalGate, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True(delay) => self.received_string = String::from("True * ") + &delay.to_string(),
            Output::False(delay) => self.received_string = String::from("False * ") + &delay.to_string()
        };
    }

    fn visit_not_node(&mut self, node: &Node) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a NotGate, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True(delay) => self.received_string = String::from("|-> * ") + &delay.to_string(),
            Output::False(delay) => self.received_string = String::from("|X> * ") + &delay.to_string()
        };
    }

    fn visit_and_node(&mut self, node: &Node) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a AndGate, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True(delay) => self.received_string = String::from("&-& * ") + &delay.to_string(),
            Output::False(delay) => self.received_string = String::from("&X& * ") + &delay.to_string()
        };
    }

    fn visit_or_node(&mut self, node: &Node) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a OrGate, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True(delay) => self.received_string = String::from("|-| * ") + &delay.to_string(),
            Output::False(delay) => self.received_string = String::from("|X| * ") + &delay.to_string()
        };
    }

    fn visit_probe_node(&mut self, node: &Node) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a ProbeGate, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True(delay) => self.received_string = String::from("(-) * ") + &delay.to_string(),
            Output::False(delay) => self.received_string = String::from("(X) * ") + &delay.to_string()
        };
    }

}

pub trait Visitable {
    fn accept_visitor(&self, v: &mut Box<dyn VisitorTrait>);
}
impl<'a> Visitable for Node {
    fn accept_visitor(&self, v: &mut Box<dyn VisitorTrait>) {
        match self {
            Node::Not(_) => v.visit_not_node(self),
            Node::Signal(_) => v.visit_signal_node(self),
            Node::And(_) => v.visit_and_node(self),
            Node::Or(_) => v.visit_or_node(self),
            Node::Probe(_) => v.visit_probe_node(self)
        }
    }
}
