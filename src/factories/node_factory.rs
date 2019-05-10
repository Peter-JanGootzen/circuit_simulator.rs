use crate::models::node::NodeTrait;
use crate::models::node::NodeStruct;
use crate::models::gates::AndGate;
use crate::models::gates::NotGate;
use crate::models::gates::SignalGate;
use crate::models::output::Output;

pub struct NodeFactory;
impl NodeFactory {
    pub fn produce_node(type_name: String) -> Result<Box<dyn NodeTrait>, &'static str> {
        match type_name.as_str() {
            "INPUTHIGH" => Ok(Box::new(NodeStruct::new(SignalGate { signal: Output::True }))),
            "INPUTLOW" => Ok(Box::new(NodeStruct::new(SignalGate { signal: Output::False }))),
            "OR" => Err("Not implemented!"),
            "NOT" => Ok(Box::new(NodeStruct::new(NotGate))),
            "AND" => Ok(Box::new(NodeStruct::new(AndGate))),
            _ => Err("This type is unknown")
        }
    }
}