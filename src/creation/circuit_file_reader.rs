use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::io;

pub struct AvansCircuitReader {}
pub struct AlternativeCircuitReader {}

pub trait CircuitReaderTrait {
    /// Reads a circuit from a file and returns the nodes and their inputs into seperate lists.
    fn read_circuit(&self, filepath: String) -> Result<(HashMap<String, String>, HashMap<String, Vec<String>>), &'static str> {
        let file = match self.load_file(filepath) {
            Ok(file) => file,
            Err(_) => { return Err("Unknown IO error occured"); }
        };
        let sections = match self.separate_sections(file) {
            Ok(sections) => sections,
            Err(error) => { return Err(error); }
        };
        let node_definitions = self.parse_nodes(sections.0);
        let input_definitions = self.parse_inputs(sections.1);
        return Ok((node_definitions, input_definitions))
    }

    /// Loads the file and returns it.
    fn load_file(&self, filepath: String) -> io::Result<File> {
        return File::open(&filepath);
    }
    
    /// Separates the node definitions from the input definitions to two separate lists.
    fn separate_sections(&self, file: File) -> Result<(Vec<String>, Vec<String>), &'static str> {
        let bf = BufReader::new(file);
        let mut node_lines : Vec<String> = Vec::new();
        let mut input_lines : Vec<String> = Vec::new();
        let mut secondsection : bool = false;
        for line in bf.lines(){
           match line {
                Ok(line) => {
                    if !line.contains("#") {
                        if !secondsection {
                            if line != "" {
                                node_lines.push(line);
                            } else {
                                secondsection = true;
                            }
                        } else {
                            input_lines.push(line)
                        }
                    }                    
                }
                Err(_) => { return Err("Unable to read line") }
            }
        } 
        return Ok((node_lines, input_lines))
    }

    /// Parses nodes to a tuple (name, type of node).
    fn parse_nodes(&self, lines: Vec<String>) -> HashMap<String, String> {
        let mut nodes: HashMap<String, String> = HashMap::new();
        for line in lines {
                let node = self.parse_node_details(line);
                nodes.insert(node.0, node.1);
            }
        return nodes; 
    }

    /// Parses a single line to a usable node description.
    fn parse_node_details(&self, line: String) -> (String, String) {
        let node_strings : Vec<&str> = line.split(":").collect();
        let node_name = node_strings[0].to_string();
        let mut node_type = node_strings[1].to_string();
        node_type.remove(node_type.len() - 1);
        println!("{:?} - {:?}", node_name, node_type);
        return (node_name, node_type)
    }

    /// Parses inputs to a tuple (name, list of inputs).
    fn parse_inputs(&self, lines: Vec<String>) -> HashMap<String, Vec<String>> {
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for line in lines {
                let node = self.parse_input_details(line);
                inputs.insert(node.0, node.1);
            }
        return inputs; 
    }

    /// Parses a single line to a usable input description.
    fn parse_input_details(&self, line: String) -> (String, Vec<String>) {
        let input_strings: Vec<&str> = line.split(":").collect();
        let node_name = input_strings[0].to_string();
        let mut inputs: Vec<String> = input_strings[1].split(",").into_iter().map(|s| s.to_string()).collect();
        //Removes the last character from the last input string.
        let inputs_len = inputs.len();
        let idx = inputs[inputs_len - 1].len() -1;
        inputs[inputs_len - 1].remove(idx);
        println!("name: {:?}", node_name);
        for input in inputs.iter() {
            println!("input: {:?}", input);
        }
        return (node_name, inputs);
    }
}

impl CircuitReaderTrait for AvansCircuitReader {}

impl CircuitReaderTrait for AlternativeCircuitReader
{
    fn parse_node_details(&self, line: String) -> (String, String) {
        let node_strings : Vec<&str> = line.split("|").collect();
        let node_name = node_strings[0].to_string();
        let mut node_type = node_strings[1].to_string();
        node_type.remove(node_type.len() - 1);
        return (node_name, node_type)
    }
}
