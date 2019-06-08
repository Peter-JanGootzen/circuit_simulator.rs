use crate::circuit_builder;
use crate::circuit_builder::ColonNodeReader;
use crate::circuit_builder::PipeNodeReader;
use std::collections::HashMap;
use crate::models::node::NodeTrait;
use super::circuit_builder::NodeReaderTrait;

pub fn load_circuit(circuit_information: (Vec<String>, Vec<String>, String)) -> Result<HashMap<String, Box<dyn NodeTrait>>,  &'static str>
{
    let mut nodes: HashMap<String, Box<dyn NodeTrait>> = HashMap::new();

    let node_reader = ColonNodeReader{};
    if circuit_information.2 == ".pipe" {
       let node_reader = PipeNodeReader{};
    }

    for line in circuit_information.0 {
        if line.contains("NODE") {
            let node_details = node_reader.read_node_details(line);
            match circuit_builder::build_node(node_details.1) {
                Ok(node) => { nodes.insert(node_details.0, node); }
                Err(error) => { return Err(error) }
            }
        }
    }
    return Ok(nodes);
}
