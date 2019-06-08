use crate::NodeFactory;
use crate::models::node::NodeTrait;

pub struct PipeNodeReader {}
pub struct ColonNodeReader {}

pub trait NodeReaderTrait {
    fn read_node_details(&self, line: String) -> (String, String) {
        let node_strings : Vec<&str> = line.split(":").collect();
        let node_name = node_strings[0].to_string();
        let mut node_type = node_strings[1].to_string();
        node_type.remove(node_type.len() - 1);
        return (node_name, node_type)
    }
}

impl NodeReaderTrait for PipeNodeReader
{
    fn read_node_details(&self, line: String) -> (String, String) {
        let node_strings : Vec<&str> = line.split("|").collect();
        let node_name = node_strings[0].to_string();
        let mut node_type = node_strings[1].to_string();
        node_type.remove(node_type.len() - 1);
        return (node_name, node_type)
    }
}

impl NodeReaderTrait for ColonNodeReader {}

pub fn build_node(node_type: String) -> Result<Box<dyn NodeTrait>,  &'static str>
{
    let factory : NodeFactory = NodeFactory::new();
    return factory.produce_node(node_type.as_str());
}

//pub fn connect_nodes(lines: Vec<String>, nodes: HashMap<String, Box<dyn NodeTrait>>) -> Result<HashMap<String, Box<dyn NodeTrait>>,  &'static str>
//{
//    let node_connection : Vec<&str> = line.split(":").collect();
//    let node_inputs : Vec<&str> = line.split(",").collect();
//    return Ok(nodes);
//}
