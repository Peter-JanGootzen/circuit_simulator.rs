use crate::models::node::NodeTrait;
use crate::models::node::NodeStruct;
use crate::models::gates::AndGate;
use crate::models::gates::NotGate;
use crate::models::gates::OrGate;
use crate::models::gates::SignalGate;
use crate::models::output::Output;

use std::collections::HashMap;

pub struct NodeFactory<'a> {
    nodes: HashMap<&'a str, Box<dyn NodeTrait>>
}

impl<'a> NodeFactory<'a> {
    pub fn produce_node(&self, type_name: &str) -> Result<Box<dyn NodeTrait>, &'static str> {
        if self.nodes.contains_key(type_name) {
            Ok(self.nodes[type_name].clone())
        } else {
            Err("This type is unknown")
        }
        /*
        match type_name {
            "INPUTHIGH" => Ok(Box::new(NodeStruct::new(SignalGate { signal: Output::True }))),
            "INPUTLOW" => Ok(Box::new(NodeStruct::new(SignalGate { signal: Output::False }))),
            "OR" => Ok(Box::new(NodeStruct::new(OrGate))),
            "NOT" => Ok(Box::new(NodeStruct::new(NotGate))),
            "AND" => Ok(Box::new(NodeStruct::new(AndGate))),
            _ => Err("This type is unknown")
        }
        */
    }

    pub fn register_node_type<'b>(self, type_name: &'b str, node: Box<dyn NodeTrait>) {
        
    }

    pub fn new() -> NodeFactory<'a> {
        let mut nodes: HashMap<&'a str, Box<dyn NodeTrait>> = HashMap::new();
        nodes.insert("INPUTHIGH", Box::new(NodeStruct::new(SignalGate { signal: Output::True })));
        nodes.insert("INPUTLOW", Box::new(NodeStruct::new(SignalGate { signal: Output::False })));
        nodes.insert("NOT", Box::new(NodeStruct::new(NotGate)));
        nodes.insert("AND", Box::new(NodeStruct::new(AndGate)));
        nodes.insert("OR", Box::new(NodeStruct::new(OrGate)));
        NodeFactory {
            nodes: nodes
        }
    }
}
