mod models;
mod creation;
mod gui;
mod circuit_checking;
mod tests;

use creation::circuit_file_reader::AvansCircuitReader;
use creation::circuit_file_reader::CircuitReaderTrait;
use creation::circuit_loader;

use models::circuit::Circuit;

use std::io::stdin;
use std::io::prelude::*;
use std::cell::RefCell;
use crate::models::node::NodeTrait;

fn main() {
    let mut circuit: Option<Circuit> = None;
    let std_in = stdin();

    //gui::init(|filepath: String| {
    //    let circuit_file_reader = AvansCircuitReader{};
    //    match circuit_file_reader.read_circuit(filepath) {
    //        Ok(circuit_file) => { 
    //            circuit = Some(circuit_loader::load_circuit(circuit_file.0, circuit_file.1));

    //            return None;
    //        },
    //        Err(_) => Some("Could not load circuit!")
    //    };
    //    None
    //}, || return &circuit);
    //gtk::main();

    match load_circuit(String::from("./examplecircuit.txt")) {
        Ok(circuit) => {
            let output_nodess = circuit.get_output_nodes();
            for node in output_nodess.iter() {
                println!("{:?}", node.borrow_mut().get_output());
            }
            println!("{}", gui::simulate(&Some(circuit)));
        },
        Err(message) => panic!(message)
    };

//    for line in std_in.lock().lines() {
//        match line {
//            Ok(text) => {
//                if text == String::from("exit") {
//                    std::process::exit(0);
//                } else if text == String::from("help") {
//                    panic!("Created by Peter-Jan and Bram-Boris");
//                } else if text.starts_with("f ") {
//                    let filepath = text.split_at(2).1;
//                    match load_circuit(filepath.to_string()) {
//                        Ok(circuit) => {
//                            println!("{}", gui::simulate(&Some(circuit)));
//                        },
//                        Err(message) => panic!(message)
//                    };
//                }
//            },
//            Err(_) => {
//                println!("ERRR");
//            }
//        };
//    }
}
 
fn load_circuit(filepath: String) -> Result<Circuit, &'static str> {
    let circuit_file_reader = AvansCircuitReader{};
    match circuit_file_reader.read_circuit(filepath) {
        Ok(circuit_file) => { 
                return Ok(circuit_loader::load_circuit(circuit_file.0, circuit_file.1));
            }
        Err(_) => { return Err("Could not load circuit!"); }
    };
}
