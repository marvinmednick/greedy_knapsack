use std::path::Path;
use std::fs::File;

use std::io::{prelude::*, BufReader,BufRead};
use regex::Regex;
//use std::io;

use log::{ info, /*error, */ debug, /*warn,*/ trace };

mod cmd_line;
use crate::cmd_line::CommandArgs;

//mod log_string_vec;
//use crate::log_string_vec::{info_vec,debug_vec};


mod knapsack;
use crate::knapsack::KnapsackInfo;

fn process_knapsack(file: &mut File) {

    let mut reader = BufReader::new(file);

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();
    info!("First Input Line is \'{}\'",line);
    let first_line_regex = Regex::new(r"\s*(?P<size>\d+)\s+(?P<num_vertex>\d+)\s+.*$").unwrap();
    let caps = first_line_regex.captures(&line).unwrap();
    let knapsack_size = caps["size"].parse::<u64>().unwrap(); 
    let _num_vertex = caps["num_vertex"].parse::<u32>().unwrap(); 
    
    info!("Setting up knapsack with size {} and expecting {} vertexes",knapsack_size, _num_vertex);
    let mut k = KnapsackInfo::new(knapsack_size);

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let mut line_data = line.unwrap();
        debug!("Processing {} {}",_count, line_data);
        if _count % 50 == 0 {
            info!("Proccesed {}", _count);
        }
        let line_regex = Regex::new(r"\s*(?P<value>\d+)\s+(?P<weight>\d+)*$").unwrap();
        trace!("Line is {}",line_data);
        let caps = line_regex.captures(&line_data).unwrap();
        trace!("Caps is {:#?}",caps);
        let value = caps["value"].parse::<u32>().unwrap(); 
        let weight = caps["weight"].parse::<u64>().unwrap(); 
        k.add_vertex(value,weight);
    }
    k.process();
}

fn main() {

    env_logger::init();

    println!("Hello");
    let cmd_line = CommandArgs::new();
    println!("Hello2");

    debug!("The Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    info!("Starting process");
    process_knapsack(&mut file);
    info!("Done with process");
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let _h = KnapsackInfo::new(100);
        debug!("Testing");
    }

 }
