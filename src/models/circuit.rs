use std::cell::RefCell;
use std::rc::Rc;
use crate::models::node::Node;
use crate::models::node::NodeTrait;
use crate::models::output::Output;

#[derive(Clone)]
pub struct Circuit {
    nodes: RefCell<Vec<Rc<RefCell<Node>>>>,
    output_nodes: RefCell<Vec<Rc<RefCell<Node>>>>
}

//impl NodeTrait for Circuit {
//    fn get_output(&self) -> Output {
//        return self..get_output();
//    }
//}

impl Circuit {
    pub fn new(nodes: RefCell<Vec<Rc<RefCell<Node>>>>, output_nodes: RefCell<Vec<Rc<RefCell<Node>>>>) -> Circuit {
        Circuit {
            nodes: nodes,
            output_nodes: output_nodes
        }
    }
    pub fn get_output_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        return self.output_nodes.borrow().iter().map(|node| node.clone()).collect();
    }
    pub fn get_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        return self.nodes.borrow().iter().map(|node| node.clone()).collect();
    }
    pub fn get_delay(&self) -> Vec<u32> {
        let mut delays: Vec<u32> = Vec::new();
        self.get_output_nodes().iter().map(|node| delays.push(match node.borrow().get_output() {
            Output::True(delay) => delay.clone(),
            Output::False(delay) => delay.clone(),
        }));
        return delays;
    }
}

