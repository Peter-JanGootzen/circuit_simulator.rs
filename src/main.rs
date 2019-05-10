mod models;
mod gui;

use models::NodeTrait;
use models::NodeStruct;
use models::AndGate;
use models::NotGate;
use models::SignalGate;
use models::Output;

fn main() {
    gui::init();
    println!("Hello, world!");
    gtk::main();

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

    let mut and_node = NodeStruct {
        inputs: vec![Box::new(signal_true_node), Box::new(not_node)],
        gate: AndGate
    };

    println!("{:?}", and_node.get_output());
}
