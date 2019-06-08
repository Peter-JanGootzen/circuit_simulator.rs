use crate::models::node::NodeTrait;
use crate::models::node::NodeStruct;
use crate::models::gates::AndGate;
use crate::models::gates::NotGate;
use crate::models::gates::OrGate;
use crate::models::gates::SignalGate;
use crate::models::output::Output;

use std::collections::HashMap;

pub struct NodeFactory {
    nodes: HashMap<String, Box<dyn NodeTrait>>
}

impl NodeFactory {
    pub fn produce_node(&self, type_name: &str) -> Result<Box<dyn NodeTrait>, &'static str> {
        if self.nodes.contains_key(type_name) {
            Ok(self.nodes[type_name].clone())
        } else {
            Err("This type is unknown")
        }
    }

    pub fn register_node_type(&mut self, type_name: String, node: Box<dyn NodeTrait>) {
        self.nodes.insert(type_name, node);
    }

    pub fn new() -> NodeFactory {
        NodeFactory {
            nodes: HashMap::new()
        }
    }
    pub fn new_filled() -> NodeFactory {
        let mut nf = NodeFactory {
            nodes: HashMap::new()
        };

        nf.register_node_type(String::from("INPUTHIGH"), Box::new(NodeStruct::new(SignalGate { signal: Output::True })));
        nf.register_node_type(String::from("INPUTLOW"), Box::new(NodeStruct::new(SignalGate { signal: Output::False })));
        nf.register_node_type(String::from("NOT"), Box::new(NodeStruct::new(NotGate)));
        nf.register_node_type(String::from("AND"), Box::new(NodeStruct::new(AndGate)));
        nf.register_node_type(String::from("OR"), Box::new(NodeStruct::new(OrGate)));

        return nf;
    }
}
