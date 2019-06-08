use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

pub fn load_file(filepath: String) -> Result<(Vec<String>, Vec<String>, String), &'static str>
{
    match File::open(&filepath) {
        Ok(file) => { 
            let bf = BufReader::new(file);
            let mut node_lines : Vec<String> = Vec::new();
            let mut connection_lines : Vec<String> = Vec::new();
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
                                connection_lines.push(line)
                            }
                        }                    
                    }
                    Err(_) => { return Err("Unable to read line") }
                }
            }
            println!("Node lines:  {:?}", node_lines.len());
            println!("Connection lines: {:?}", connection_lines.len());
            let extension = Path::new(&filepath).extension();
            let extension = match extension {
                Some(extension) => extension.to_str().unwrap(),
                None => { return Err("Not able to load file extension") }
            };
            return Ok((node_lines, connection_lines, extension.to_string()));
        }
        Err(_) => { return Err("Unable to open file") }
    };
}
