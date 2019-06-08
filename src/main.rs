mod models;
mod factories;
mod gui;
mod circuit_checking;
mod tests;
mod circuit_file_reader;
mod circuit_builder;
mod circuit_loader;

use models::node::NodeTrait;
use models::node::NodeStruct;
use models::gates::AndGate;
use models::gates::NotGate;
use models::gates::SignalGate;
use models::output::Output;
use models::visitor::Visitor;
use models::visitor::Visitable;
use factories::node_factory::NodeFactory;
use std::ptr::eq;
use std::collections::HashMap;

fn main() {
    //let signal_false_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
    //    inputs: Vec::new(),
    //    gate: SignalGate { signal: Output::False }
    //});
    //let not_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
    //    inputs: vec![&Box::new(signal_false_node)],
    //    gate: NotGate
    //});

    //let signal_true_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
    //    inputs: Vec::new(),
    //    gate: SignalGate { signal: Output::True }
    //});
    ////signal_true_node.accept_visitor(Visitor);

    //let and_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
    //    inputs: vec![&Box::new(signal_true_node), &Box::new(not_node)],
    //    gate: AndGate
    //});
    //println!("The last Node outputs: {:?}", and_node.get_output());

    //let factory = NodeFactory::new();
    //let factory_filled = NodeFactory::new_filled();
    //let node_type = "INPUTLOW";
    //let node_from_factory = factory_filled.produce_node(node_type).unwrap();
    //let node_from_factory2 = factory_filled.produce_node(node_type).unwrap();
    //println!("We created a node with the NodeFactory with the type_name: {}, its output is: {:?}", node_type, node_from_factory.get_output());

    //println!("We created another node of that type and are they equal? {}", eq(&node_from_factory, &node_from_factory2));

    gui::init(|filepath|
              match circuit_file_reader::load_file(filepath) {
                  Ok(circuit_information) => {
                      match circuit_loader::load_circuit(circuit_information) {
                          Ok(nodes) =>  {
                              println!("Nodes created {:?}", nodes.len());
                              //match circuit_builder::connect_nodes(lines.1, nodes) {
                              //    Ok(nodes) => println!("Completed circuit with {:?} nodes.", nodes.len()),
                              //    Err(error) => { gui::show_error(error); }
                              //}

                          },
                          Err(error) => { gui::show_error(error); }
                      };

                  },
                  Err(error) => { return gui::show_error(error) }
              }
    );
    gtk::main();
    //let mut vec: Vec<&Box<dyn NodeTrait>> = Vec::new();
    //vec.push(&node_from_factory);
    //println!("{:?}", vec.contains(&&node_from_factory));

    //let asb = node_from_factory as *const Box<dyn NodeTrait>;
    //println!("{:?}", );

    //let mut hashmap: HashMap<&Box<dyn NodeTrait>, bool> = HashMap::new();
    //hashmap.insert(&node_from_factory, true);
    //hashmap.insert(&node_from_factory2, false);

    //println!("{}", hashmap[&node_from_factory]);
    //println!("{}", hashmap[&node_from_factory2]);

    //gui::init();
    //println!("Hello, world!");
    //gtk::main();
}
