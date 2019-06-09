use crate::creation::builders::circuit_builder::CircuitBuilder;
use std::collections::HashMap;
use crate::models::circuit::Circuit;
use crate::creation::factories::node_factory::LowBindingNodeFactory;
use crate::circuit_checking::strategies::lax_check_strategy::LaxCheckStrategy;
use crate::circuit_checking::check_strategy::CheckStrategy;

pub fn load_circuit<'a>(nodes: HashMap<String, String>, node_links: HashMap<String, Vec<String>>) -> Circuit {
    //doe builder shit
    let factory = LowBindingNodeFactory::new_filled();
    let strategy: Box<dyn CheckStrategy> = Box::new(LaxCheckStrategy{});
    let mut builder = CircuitBuilder::new(factory, strategy);
    for (node_name, node_type) in nodes {
        match builder.create_node(node_name, node_type) {
            Some(message) => panic!(message),
            _ => ()
        }
    }
    for (node_name, node_links) in node_links {
        builder.connect_inputs(node_name, node_links);
    }

    return builder.get_circuit();
}
