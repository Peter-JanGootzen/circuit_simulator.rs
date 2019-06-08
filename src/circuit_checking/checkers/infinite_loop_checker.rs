use crate::models::node::NodeTrait;
use crate::circuit_checking::checker::Checker;
use crate::models::circuit::Circuit;
use crate::circuit_checking::checker_message::CheckerMessage;

pub struct InfiniteLoopChecker<'a> {
    already_found: Vec<&'a Box<dyn NodeTrait>>
}
impl<'a> Checker<'a> for InfiniteLoopChecker<'a> {
    fn check(&mut self, circuit: &'a Circuit) -> Option<CheckerMessage> {
        return self.rec(circuit.get_last_node());
    }
}
impl<'a> InfiniteLoopChecker<'a> {
    fn rec(&mut self, last_node: &'a Box<dyn NodeTrait>) -> Option<CheckerMessage> {
        for node in last_node.get_inputs().iter() {
            if self.already_found.contains(&&node) {
                return Some(CheckerMessage::Error("Infinite loop detected!"));
            }
            self.already_found.push(&node);
            return self.rec(&node);
        }
        return None;
    }
    pub fn new() -> InfiniteLoopChecker<'a> {
        InfiniteLoopChecker {
            already_found: Vec::new()
        }
    }
}