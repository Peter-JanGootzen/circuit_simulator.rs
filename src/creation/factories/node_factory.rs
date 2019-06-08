use crate::models::node::NodeTrait;
use crate::models::node::NodeStruct;
use crate::models::gates::AndGate;
use crate::models::gates::NotGate;
use crate::models::gates::OrGate;
use crate::models::gates::SignalGate;
use crate::models::output::Output;

use std::collections::HashMap;

pub struct LowBindingNodeFactory {
    nodes: HashMap<String, Box<dyn NodeTrait>>
}

pub trait NodeFactoryTrait {
    fn produce_node(&self, type_name: String) -> Result<Box<dyn NodeTrait>, &'static str>;
    fn register_node_type(&mut self, type_name: String, node: Box<dyn NodeTrait>);
}

impl NodeFactoryTrait for LowBindingNodeFactory {
    fn produce_node(&self, type_name: String) -> Result<Box<dyn NodeTrait>, &'static str> {
        if self.nodes.contains_key(&type_name) {
            Ok(self.nodes[&type_name].clone())
        } else {
            Err("This type is unknown")
        }
    }

    fn register_node_type(&mut self, type_name: String, node: Box<dyn NodeTrait>) {
        self.nodes.insert(type_name, node);
    }
}

impl<'a> LowBindingNodeFactory {
    pub fn new() -> Box<dyn NodeFactoryTrait> {
        let nf: Box<dyn NodeFactoryTrait> = Box::new(LowBindingNodeFactory {
            nodes: HashMap::new()
        });
        return nf;
    }
    pub fn new_filled() -> LowBindingNodeFactory {
        let mut nf = LowBindingNodeFactory {
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
