use crate::models::node::Node;
use crate::models::circuit::Circuit;
use crate::creation::factories::node_factory::NodeFactoryTrait;
use std::collections::HashMap;
use crate::circuit_checking::check_strategy::CheckStrategy;
use std::rc::Rc;
use std::cell::RefCell;


pub struct CircuitBuilder {
    nodes: HashMap<String, Rc<RefCell<Node>>>,
    factory: Box<dyn NodeFactoryTrait>,
    check_strategy: Box<dyn CheckStrategy>,
    outputs: Vec<String>,
    inputs: Vec<String>,
    link_counter: u32
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
                self.nodes.insert(node_name, Rc::new(RefCell::new(node)));
                return None;
            },
            Err(message) => {
                return Some(message);
            },
        }
    }
    fn connect_inputs(&mut self, node: String, node_outputs: Vec<String>) -> Option<&'static str> {
        let to_add = Rc::clone(&self.nodes[&node]);
        for node_output in node_outputs {
            if !self.outputs.contains(&node_output) {
                self.outputs.push(node_output.clone());
            }

            if !self.inputs.contains(&node) {
                self.inputs.push(node.clone());
            }

            match self.nodes.get(&node_output.clone()) {
                Some(mut_node) => {
                        self.link_counter += 1;
                        mut_node.borrow_mut().add_input(Rc::clone(&to_add));
                },
                None => ()
            }
            println!("{:?} links were created", self.link_counter);
            // self.nodes.insert(node_link.clone(), to_add);
            //mut_node.add_input(new_node);
            //*self.nodes.entry(&node_name.clone()).or_insert(42).add_input(&new_node);
            //self.nodes[&node_name.clone()].add_input(&new_node);
        }
        return None;
    }
    fn get_circuit(&mut self) -> Circuit {
        let output_nodes: RefCell<Vec<Rc<RefCell<Node>>>> = RefCell::new(Vec::new());
        for output in self.outputs.iter() {
            if !self.inputs.contains(&output) {
                output_nodes.borrow_mut().push(self.nodes[output].clone());
            }
        };
        // for (node_name, is_no_output) in self.nodes_is_no_ouput.iter() {
        //     if !*is_no_output {
        //         // Extra dereferencing and referencing for increased performance.
        //         let new_f = self.nodes.remove(node_name);
        //         match new_f {
        //             Some(node) => output_nodes.borrow_mut().push(node),
        //             None => println!("not found")
        //         };
        //     }
        // };
        let mut nodes: RefCell<Vec<Rc<RefCell<Node>>>> = RefCell::new(vec![]);
        for (_, node) in self.nodes.iter() {
            nodes.borrow_mut().push(Rc::clone(node));
        }
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
            check_strategy: checker_strategy,
            inputs: Vec::new(),
            outputs: Vec::new(),
            link_counter: 0
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
