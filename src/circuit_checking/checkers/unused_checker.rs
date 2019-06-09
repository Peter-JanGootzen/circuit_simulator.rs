//use std::collections::HashMap;
//use crate::models::node::Node;
//use crate::circuit_checking::checker::Checker;
//use crate::models::circuit::Circuit;
//use crate::circuit_checking::checker_message::CheckerMessage;
//use std::rc::Rc;
//use std::borrow::Borrow;
//use std::cell::RefCell;

//pub struct UnusedChecker {
    //nodes_found: HashMap<Rc<RefCell<Node>>, bool>
//}

//impl Checker for UnusedChecker {
    //fn check(&mut self, circuit: &Circuit) -> Option<CheckerMessage> {
        //for node in circuit.get_nodes().iter() {
            //self.nodes_found.insert(node.clone().into_inner(), false);
        //}


        //let circuit_output_nodes = circuit.get_output_nodes();
        //for node in circuit_output_nodes.iter() {
            //self.rec(node.clone());
        //}

        //for (_, found) in self.nodes_found.iter() {
            //if !found {
                //return Some(CheckerMessage::Warning("Unused node detected"));
            //}
        //}

        //return None;
    //}

//}

//impl UnusedChecker {
    //fn rec(&mut self, current_node: Rc<RefCell<Node>>) {
        //self.nodes_found.insert(current_node.clone(), true);
        //let input_nodes = current_node.get_input_nodes();
        //match input_nodes {
            //Some(nodes) => {
                //for node in nodes.iter() {
                    //self.rec(node.clone());
                //}
            //},
            //None => ()
        //}
    //}
    //pub fn new() -> UnusedChecker {
        //return UnusedChecker {
            //nodes_found: HashMap::new()
        //}
    //}
//}
