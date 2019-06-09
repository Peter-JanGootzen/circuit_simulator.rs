use crate::models::node::Node;
use crate::models::circuit::Circuit;
use crate::creation::factories::node_factory::NodeFactoryTrait;
use std::collections::HashMap;
use crate::circuit_checking::check_strategy::CheckStrategy;
use std::rc::Rc;
use std::cell::RefCell;


pub struct CircuitBuilder {
    nodes: HashMap<String, Rc<Node>>,
    nodes_is_no_ouput: HashMap<String, bool>,
    factory: Box<dyn NodeFactoryTrait>,
    check_strategy: Box<dyn CheckStrategy>
}
pub trait CircuitBuilderTrait {
    fn create_node(&mut self, node_name: String, node_type: String) -> Option<&'static str>;
    fn connect_inputs(&mut self, node_name: String, node_links: Vec<String>) -> Option<&'static str>;
    fn get_circuit(&mut self) -> Circuit;
}

impl CircuitBuilderTrait for CircuitBuilder {
    fn create_node(&mut self, node_name: String, node_type: String) -> Option<&'static str> {
        match self.factory.produce_node(node_type) {
            Ok(node) => {
                self.nodes.insert(node_name, Rc::new(node));
                return None;
            },
            Err(message) => {
                return Some(message);
            },
        }
    }
    fn connect_inputs(&mut self, node_name: String, node_links: Vec<String>) -> Option<&'static str> {
        for node_link in node_links.iter() {
            self.nodes_is_no_ouput.insert(node_link.clone(), true);
            let to_add =  match self.nodes.remove(&node_link.clone()) {
                Some(node) => node,
                None => return Some("Could not find node in HashMap, if this error occured just give up.")
            };
            match self.nodes.get_mut(&node_name.clone()) {
                Some(mut_node) => match Rc::get_mut(mut_node) {
                    Some(rc_mut_node) => rc_mut_node.add_input(to_add.clone()),
                    None => return Some("Could not get a mut lock on the node")
                }
                None => ()
            }
            self.nodes.insert(node_link.clone(), to_add);
            //mut_node.add_input(new_node);
            //*self.nodes.entry(&node_name.clone()).or_insert(42).add_input(&new_node);
            //self.nodes[&node_name.clone()].add_input(&new_node);
        }
        return None;
    }
    fn get_circuit(&mut self) -> Circuit {
        let output_nodes: RefCell<Vec<Rc<Node>>> = RefCell::new(Vec::new());
        for (node_name, is_no_output) in self.nodes_is_no_ouput.iter() {
            if !*is_no_output {
                // Extra dereferencing and referencing for increased performance.
                let new_f = self.nodes.remove(node_name);
                match new_f {
                    Some(node) => output_nodes.borrow_mut().push(node),
                    None => println!("not found")
                };
            }
        };
        let nodes: RefCell<Vec<Rc<Node>>> = RefCell::new(self.nodes.iter().map(|(_, node)| node.clone()).collect());
        let c = Circuit::new(nodes, output_nodes);
        self.check_strategy.check(&c);
        //check strategy

        return c;
    }
}
impl CircuitBuilder  {
    pub fn new(factory: Box<dyn NodeFactoryTrait>, checker_strategy: Box<dyn CheckStrategy>) -> Box<dyn CircuitBuilderTrait> {
        let cb: Box<dyn CircuitBuilderTrait> = Box::new(CircuitBuilder {
            factory: factory,
            nodes: HashMap::new(),
            nodes_is_no_ouput: HashMap::new(),
            check_strategy: checker_strategy
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
