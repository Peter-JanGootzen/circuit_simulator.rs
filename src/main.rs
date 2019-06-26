extern crate clap;

mod models;
mod creation;
mod gui;
mod circuit_checking;
mod tests;

use clap::App;
use clap::Arg;

use creation::circuit_file_reader::AvansCircuitReader;
use creation::circuit_file_reader::CircuitReaderTrait;
use creation::circuit_loader;
use models::circuit::Circuit;
use models::node::NodeTrait;

fn main() {
    let matches = App::new("circuit_simulator.rs")
        .version("0.9")
        .about("")
        .author("Peter-Jan Gootzen & Bram-Boris Meerlo")
        .arg(Arg::with_name("circuit_file_name")
             .help("The circuit that will get loaded")
             .required(true)
             .index(1))
        .get_matches();

    let circuit_file_name = matches.value_of("circuit_file_name").unwrap().to_string();
    match load_circuit(circuit_file_name) {
        Ok(circuit) => {
            let output_nodess = circuit.get_output_nodes();
            for node in output_nodess.iter() {
                println!("{:?}", node.borrow_mut().get_output());
            }
            println!("{}", gui::simulate(&Some(circuit)));
        },
        Err(message) => panic!(message)
    };
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
