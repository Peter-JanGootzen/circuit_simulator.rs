use crate::models::node::Node;
use crate::circuit_checking::checker::Checker;
use crate::models::circuit::Circuit;
use crate::circuit_checking::checker_message::CheckerMessage;
use std::rc::Rc;

pub struct InfiniteLoopChecker {
    already_found: Vec<Rc<Node>>
}
impl Checker for InfiniteLoopChecker {
    fn check(&mut self, circuit: &Circuit) -> Option<CheckerMessage> {
        let circuit_output_nodes = circuit.get_output_nodes();
        for node in circuit_output_nodes.iter() {
            let message_option = self.rec(node.clone());
            match message_option {
                Some(message) => return Some(message),
                None => ()
            }
        }
        return None;
    }
}
impl InfiniteLoopChecker {
    fn rec(&mut self, last_node: Rc<Node>) -> Option<CheckerMessage> {
        let input_nodes_option = last_node.get_input_nodes();
        match input_nodes_option {
            Some(input_nodes) => {
                for node in input_nodes.iter() {
                    if self.already_found.contains(&node.clone()) {
                        return Some(CheckerMessage::Error("Infinite loop detected!"));
                    }
                    self.already_found.push(node.clone());
                    return self.rec(node.clone());
                }
            },
            None => ()
        }
        return None;
    }
    pub fn new() -> InfiniteLoopChecker {
        InfiniteLoopChecker {
            already_found: Vec::new()
        }
    }
}