use crate::creation::builders::circuit_builder::CircuitBuilder;
use crate::creation::builders::circuit_builder::CircuitBuilderTrait;
use std::collections::HashMap;
use crate::models::circuit::Circuit;
use crate::creation::factories::node_factory::LowBindingNodeFactory;
use crate::creation::factories::node_factory::NodeFactoryTrait;
use crate::circuit_checking::strategies::lax_check_strategy::LaxCheckStrategy;
use crate::circuit_checking::check_strategy::CheckStrategy;

pub fn load_circuit<'a>(nodes: HashMap<String, String>, node_links: HashMap<String, Vec<String>>) -> Circuit {
    //doe builder shit
    let factory: Box<dyn NodeFactoryTrait> = Box::new(LowBindingNodeFactory::new_filled());
    let mut builder = CircuitBuilder::new(factory);
    for (node_name, node_type) in nodes {
        builder.create_node(node_name, node_type);
    }
    for (node_name, node_links) in node_links {
        builder.connect_inputs(node_name, node_links);
    }

    let strategy: Box<dyn CheckStrategy> = Box::new(LaxCheckStrategy{});
    return builder.get_circuit(strategy);
}
