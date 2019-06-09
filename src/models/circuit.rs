use std::cell::RefCell;
use std::rc::Rc;
use crate::models::node::Node;

#[derive(Clone)]
pub struct Circuit {
    nodes: RefCell<Vec<Rc<Node>>>,
    output_nodes: RefCell<Vec<Rc<Node>>>
}

//impl NodeTrait for Circuit {
//    fn get_output(&self) -> Output {
//        return self..get_output();
//    }
//}

impl Circuit {
    pub fn new(nodes: RefCell<Vec<Rc<Node>>>, output_nodes: RefCell<Vec<Rc<Node>>>) -> Circuit {
        Circuit {
            nodes: nodes,
            output_nodes: output_nodes
        }
    }
    pub fn get_output_nodes(&self) -> Vec<Rc<Node>> {
        return self.output_nodes.borrow().iter().map(|node| node.clone()).collect();
    }
    pub fn get_nodes(&self) -> Vec<Rc<Node>> {
        return self.nodes.borrow().iter().map(|node| node.clone()).collect();
    }
}

