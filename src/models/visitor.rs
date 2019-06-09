use crate::models::node::Node;
use crate::models::node::NodeTrait;
use crate::models::output::Output;
use std::rc::Rc;
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
    fn visit_signal_node(&mut self, node: Rc<Node>);
    fn visit_not_node(&mut self, node: Rc<Node>);
    fn visit_and_node(&mut self, node: Rc<Node>);
    fn visit_or_node(&mut self, node: Rc<Node>);
    fn visit_probe_node(&mut self, node: Rc<Node>);
    fn get_output(&self) -> String;
}
impl VisitorTrait for ConcreteVisitor {
    fn get_output(&self) -> String {
        self.received_string.clone()
    }
    fn visit_signal_node(&mut self, node: Rc<Node>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True => self.received_string = String::from("True"),
            Output::False => self.received_string =String::from("False")
        };
    }

    fn visit_not_node(&mut self, node: Rc<Node>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True => self.received_string = String::from("|->"),
            Output::False => self.received_string = String::from("|X>")
        };
    }

    fn visit_and_node(&mut self, node: Rc<Node>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True => self.received_string = String::from("&-&"),
            Output::False => self.received_string = String::from("&X&")
        };
    }

    fn visit_or_node(&mut self, node: Rc<Node>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True => self.received_string = String::from("|-|"),
            Output::False => self.received_string = String::from("|X|")
        };
    }

    fn visit_probe_node(&mut self, node: Rc<Node>) {
        // Do something with the signal gate
        println!("I am a visitor and have just visited a Nodestruct<SignalGate>, it's output is: {:?}", node.get_output());
        match node.get_output() {
            Output::True => self.received_string = String::from("(-)"),
            Output::False => self.received_string = String::from("(X)")
        };
    }

}

pub trait Visitable {
    fn accept_visitor(&self, v: &mut Box<dyn VisitorTrait>);
}
impl<'a> Visitable for Rc<Node> {
    fn accept_visitor(&self, v: &mut Box<dyn VisitorTrait>) {
        match self.borrow() {
            Node::Not(_) => v.visit_not_node(self.clone()),
            Node::Signal(_) => v.visit_signal_node(self.clone()),
            Node::And(_) => v.visit_and_node(self.clone()),
            Node::Or(_) => v.visit_or_node(self.clone()),
            Node::Probe(_) => v.visit_probe_node(self.clone())
        }
    }
}
