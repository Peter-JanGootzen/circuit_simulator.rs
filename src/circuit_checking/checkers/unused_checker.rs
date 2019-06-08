use std::collections::HashMap;
use crate::models::node::NodeTrait;
use crate::circuit_checking::checker::Checker;
use crate::models::circuit::Circuit;
use crate::circuit_checking::checker_message::CheckerMessage;

pub struct UnusedChecker<'a> {
    nodes_found: HashMap<&'a Box<dyn NodeTrait>, bool>
}

impl<'a> Checker<'a> for UnusedChecker<'a> {
    fn check(&mut self, circuit: &'a Circuit) -> Option<CheckerMessage> {
        for node in circuit.nodes.iter() {
            self.nodes_found.insert(&node, false);
        }

        self.rec(circuit.get_last_node());

        for (_, found) in self.nodes_found.iter() {
            if !found {
                return Some(CheckerMessage::Warning("Unused node detected"));
            }
        }

        return None;
    }

}

impl<'a> UnusedChecker<'a> {
    fn rec(&mut self, current_node: &'a Box<dyn NodeTrait>) {
        self.nodes_found.insert(current_node, true);
        let input_nodes = current_node.get_inputs();
        for node in input_nodes.iter() {
            self.rec(&mut &node);
        }
    }
    pub fn new() -> UnusedChecker<'a> {
        return UnusedChecker {
            nodes_found: HashMap::new()
        }
    }
}
