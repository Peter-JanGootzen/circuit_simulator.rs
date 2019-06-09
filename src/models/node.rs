extern crate objekt_clonable;
extern crate mopa;
use super::output::Output;
use std::hash::Hash;
use std::hash::Hasher;
//use std::collections::hash_map::DefaultHasher;
use std::mem::transmute;
use std::rc::Rc;
use crate::models::gates::NotGate;
use crate::models::gates::SignalGate;
use crate::models::gates::OrGate;
use crate::models::gates::AndGate;
use crate::models::gates::ProbeGate;

//fn create_key(t: &dyn NodeTrait) -> [usize; 2] {
//    unsafe {
//        std::mem::transmute::<&dyn NodeTrait, [usize; 2]>(t)
//    }
//}

#[derive(Clone)]
pub enum Node {
    Not(NotGate),
    Signal(SignalGate),
    Or(OrGate),
    And(AndGate),
    Probe(ProbeGate)
}
impl NodeTrait for Node {
    fn get_output(&self) -> Output {
        match self {
            Node::Not(gate) => gate.get_output(),
            Node::Signal(gate) => gate.get_output(),
            Node::Or(gate) => gate.get_output(),
            Node::And(gate) => gate.get_output(),
            Node::Probe(gate) => gate.get_output()
        }
    }
}

impl Node {
    pub fn add_input(&mut self, node: Rc<Node>) {
        match self {
            Node::Not(gate) => gate.inputs.borrow_mut().push(node),
            Node::And(gate) => gate.inputs.borrow_mut().push(node),
            Node::Or(gate) => gate.inputs.borrow_mut().push(node),
            Node::Probe(gate) => gate.inputs.borrow_mut().push(node),
            _ => ()
        }
    }
    pub fn get_input_nodes(&self) -> Option<Vec<Rc<Node>>> {
        match self {
            Node::Not(gate) => {
                let inputs: Vec<Rc<Node>> = gate.inputs.borrow().iter().map(|node| node.clone()).collect();
                Some(inputs.iter().map(|node| node.clone()).collect())
            }
            Node::Signal(_) => None,
            _ => None
        }
    }
}

pub trait NodeTrait {
    fn get_output(&self) -> Output;
}

pub struct CompositeNodeStruct {
    pub child_node: Rc<Node>
}

trait DataPointer<T: ?Sized> {
    fn get_memory_key(&self) -> i32;
}
impl<'a> DataPointer<Node> for Node {
    fn get_memory_key(&self) -> i32 {
        unsafe {
            transmute::<&Node, [i32; 2]>(&*self)[0]
        }
    }
}

impl<'a> PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        //return create_key(&**self)[0] == create_key(&**other)[0];
        return self.get_memory_key() == other.get_memory_key();
        //let x = std::ptr::eq(&self, &other);
        //return x;
        //return self.as_ref() == other.as_ref();
    }
}
impl<'a> Eq for Node {}
impl<'a> Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return std::ptr::hash(self, state);
    }
}

