use crate::models::node::NodeTrait;
use crate::models::node::NodeStruct;
use crate::models::gates::NotGate;
use crate::models::circuit::Circuit;
use crate::creation::factories::node_factory::NodeFactoryTrait;
use std::collections::HashMap;
use crate::circuit_checking::check_strategy::CheckStrategy;
use std::any::Any;

pub struct CircuitBuilder {
    nodes: HashMap<String, Box<dyn NodeTrait>>,
    nodes_is_no_ouput: HashMap<String, bool>,
    factory: Box<dyn NodeFactoryTrait>
}
pub trait CircuitBuilderTrait {
    fn create_node(&mut self, node_name: String, node_type: String) -> Option<&'static str>;
    fn connect_inputs(&mut self, node_name: String, node_links: Vec<String>) -> Option<&'static str>;
    fn get_circuit(&mut self, check_strategy: Box<dyn CheckStrategy>) -> Circuit;
    fn create_and_link(&mut self,nodes: HashMap<String, String>, node_links: HashMap<String, Vec<String>>) -> Option<&'static str>;
}

impl CircuitBuilderTrait for CircuitBuilder {
    fn create_node(&mut self, node_name: String, node_type: String) -> Option<&'static str> {
        match self.factory.produce_node(node_type) {
            Ok(node) => {
                self.nodes.insert(node_name, node);
                return None;
            },
            Err(message) => {
                return Some(message);
            },
        }
    }
    fn connect_inputs(&mut self, node_name: String, node_links: Vec<String>) -> Option<&'static str> {
        for node_link in node_links.iter() {
            unsafe {
                self.nodes_is_no_ouput[node_link] = true;
                let x = self.nodes[&node_name];
                let y = std::mem::transmute::<Box<dyn NodeTrait>, NodeStruct<NotGate>>(x);
                y.unwrap().add_input(&self.nodes[node_link]);
            }
        }
        return None;
    }
    fn create_and_link(&mut self,nodes: HashMap<String, String>, node_links: HashMap<String, Vec<String>>) -> Option<&'static str> {
        for (node_name, node_type) in nodes {
            builder.create_node(node_name, node_type);
        }
        for (node_name, node_links) in node_links {
            builder.connect_inputs(node_name, node_links);
        }
        match self.factory.produce_node(node_type) {
            Ok(node) => {
                self.nodes.insert(node_name, node);
                return None;
            },
            Err(message) => {
                return Some(message);
            },
        }
    }
    fn get_circuit(&mut self, check_strategy: Box<dyn CheckStrategy>) -> Circuit {
        let f: Vec<Box<dyn NodeTrait>> = Vec::new();
        for (node_name, is_no_output) in self.nodes_is_no_ouput.into_iter() {
            if is_no_output {
                f.push(self.nodes[&node_name]);
            }
        };
        let nodes: Vec<Box<dyn NodeTrait>> = self.nodes.into_iter().map(|(_, node)| node).collect();
        let c = Circuit {
            nodes: nodes,
            output_nodes: f
        };
        check_strategy.check(&c);

        return c;
    }
}
impl CircuitBuilder {
    pub fn new(factory: Box<dyn NodeFactoryTrait>) -> Box<dyn CircuitBuilderTrait> {
        let cb: Box<dyn CircuitBuilderTrait> = Box::new(CircuitBuilder {
            factory: factory,
            nodes: HashMap::new(),
            nodes_is_no_ouput: HashMap::new()
        });
        return cb;
    }
}
//pub fn connect_nodes(lines: Vec<String>, nodes: HashMap<String, Box<dyn NodeTrait>>) -> Result<HashMap<String, Box<dyn NodeTrait>>,  &'static str>
//{
//    let node_connection : Vec<&str> = line.split(":").collect();
//    let node_inputs : Vec<&str> = line.split(",").collect();
//    return Ok(nodes);
//}
