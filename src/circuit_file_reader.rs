use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::models::node::NodeTrait;
use crate::factories::node_factory::NodeFactory;
use std::collections::HashMap;

pub fn load_file(filepath: String) -> Result<File, &'static str>
{
    let factory : NodeFactory = NodeFactory::new();
    let file = match File::open(filepath) {
        Ok(file) => { file }
        Err(_) => { return Err("Unable to open file") }
    };
    let bf = BufReader::new(&file);
    let mut nodes: HashMap<String, Box<dyn NodeTrait>> = HashMap::new();
    for line in bf.lines() {
        match line {
            Ok(line) => {
                if line.contains("#") {
                    //println!("{:?}", "Nice, found a comment");
                } else if line.contains("NODE") {
                    println!("{:?}", line);
                    let node_strings : Vec<&str> = line.split(":").collect();
                    let node_name = node_strings[0];
                    let mut node_type = node_strings[1].to_string();
                    node_type.remove(node_type.len() - 1);
                    match factory.produce_node(node_type.as_str()) {
                        Ok(node) => { nodes.insert(node_name.to_string(), node); }
                        Err(error) => { return Err(error) }
                    }
                }
            }
            Err(_) => { return Err("Unable to read line") }
        }
    }
    println!("{:?}", nodes.len());
    return Ok(file);
}
