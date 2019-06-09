use crate::models::output::Output;
use crate::models::node::Node;
use crate::models::gates::signal_gate::SignalGate;
use crate::models::gates::not_gate::NotGate;
use crate::models::gates::or_gate::OrGate;
use crate::models::gates::and_gate::AndGate;
use crate::models::gates::probe_gate::ProbeGate;
use std::cell::RefCell;


use std::collections::HashMap;

pub struct LowBindingNodeFactory {
    nodes: HashMap<String, Node>
}

pub trait NodeFactoryTrait {
    fn produce_node(&self, type_name: String) -> Result<Node, &'static str>;
    fn register_node_type(&mut self, type_name: String, node: Node);
}

impl NodeFactoryTrait for LowBindingNodeFactory {
    fn produce_node(&self, type_name: String) -> Result<Node, &'static str> {
        if self.nodes.contains_key(&type_name) {
            Ok(self.nodes[&type_name].clone())
        } else {
            Err("This type is unknown to the factory")
        }
    }

    fn register_node_type(&mut self, type_name: String, node: Node) {
        self.nodes.insert(type_name, node);
    }
}

impl<'a> LowBindingNodeFactory {
    pub fn new() -> LowBindingNodeFactory {
        LowBindingNodeFactory {
            nodes: HashMap::new()
        }
    }
    pub fn new_filled() -> Box<dyn NodeFactoryTrait> {
        let mut nf: Box<dyn NodeFactoryTrait> = Box::new(LowBindingNodeFactory {
            nodes: HashMap::new()
        });

        nf.register_node_type(String::from("INPUT_HIGH"), Node::Signal(SignalGate { signal: Output::True(0) }));
        nf.register_node_type(String::from("INPUT_LOW"), Node::Signal(SignalGate { signal: Output::False(0) }));
        nf.register_node_type(String::from("PROBE"), Node::Probe(ProbeGate { inputs: RefCell::new(Vec::new())}));
        nf.register_node_type(String::from("NOT"), Node::Not(NotGate { inputs: RefCell::new(Vec::new())}));
        nf.register_node_type(String::from("AND"), Node::And(AndGate { inputs: RefCell::new(Vec::new())}));
        nf.register_node_type(String::from("OR"), Node::Or(OrGate { inputs: RefCell::new(Vec::new())}));
        return nf;
    }
}

