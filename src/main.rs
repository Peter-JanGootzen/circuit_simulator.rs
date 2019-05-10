mod models;
mod factories;
mod gui;

use models::node::NodeTrait;
use models::node::NodeStruct;
use models::gates::AndGate;
use models::gates::NotGate;
use models::gates::SignalGate;
use models::output::Output;
use models::visitor::Visitor;
use models::visitor::Visitable;
use factories::node_factory::NodeFactory;

fn main() {
    let signal_false_node = NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::False }
    };
    let not_node = NodeStruct {
        inputs: vec![Box::new(signal_false_node)],
        gate: NotGate
    };

    let signal_true_node = NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::True }
    };
    signal_true_node.accept_visitor(Visitor);

    let and_node = NodeStruct {
        inputs: vec![Box::new(signal_true_node), Box::new(not_node)],
        gate: AndGate
    };
    println!("The last Node outputs: {:?}", and_node.get_output());

    let node_type = "INPUTLOW";
    let node_from_factory = NodeFactory::produce_node(node_type.to_string()).unwrap();
    println!("We created a node with the NodeFactory with the type_name: {}, its output is: {:?}", node_type, node_from_factory.get_output());

    gui::init();
    println!("Hello, world!");
    gtk::main();
}
