extern crate objekt;
use super::output::Output;
use std::hash::Hash;
use std::hash::Hasher;
//use std::collections::hash_map::DefaultHasher;
use std::mem::transmute;

//fn create_key(t: &dyn NodeTrait) -> [usize; 2] {
//    unsafe {
//        std::mem::transmute::<&dyn NodeTrait, [usize; 2]>(t)
//    }
//}

pub trait NodeTrait: objekt::Clone  {
    fn get_output(&self) -> Output;
    fn get_inputs(&self) -> &Vec<&Box<dyn NodeTrait>>;
}
objekt::clone_trait_object!(NodeTrait);

#[derive(Clone)]
pub struct NodeStruct<'a, Gate> {
    pub inputs: Vec<&'a Box<dyn NodeTrait>>,
    pub gate: Gate,
}
impl<'a, Gate> NodeStruct<'a, Gate> {
    pub fn new(gate: Gate) -> NodeStruct<'a, Gate> {
        NodeStruct {
            inputs: Vec::new(),
            gate: gate
        }
    }
}

pub struct CompositeNodeStruct {
    pub child_node: Box<dyn NodeTrait>
}


trait DataPointer<T: ?Sized> {
    fn get_memory_key(&self) -> usize;
}
impl DataPointer<dyn NodeTrait> for Box<dyn NodeTrait> {
    fn get_memory_key(&self) -> usize {
        unsafe {
            transmute::<&dyn NodeTrait, [usize; 2]>(&**self)[0]
        }
    }
}

impl PartialEq for Box<dyn NodeTrait> {
    fn eq(&self, other: &Self) -> bool {
        //return create_key(&**self)[0] == create_key(&**other)[0];
        return self.get_memory_key() == other.get_memory_key();
        //let x = std::ptr::eq(&self, &other);
        //return x;
        //return self.as_ref() == other.as_ref();
    }
}
impl Eq for Box<dyn NodeTrait> {}
impl Hash for Box<dyn NodeTrait> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return std::ptr::hash(self, state);
    }
}

