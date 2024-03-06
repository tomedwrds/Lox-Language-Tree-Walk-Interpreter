use std::fs;

use crate::virtual_machine::{interpret_vm};

mod assignment;
mod block;
mod bool;
mod comments;
mod variable;
mod for_loop;


fn run_from_file(file: &str) -> Vec<String> {
    let result = interpret_vm(fs::read_to_string(file.to_string()).expect("Error: file doesnt exist"), false);  
    return result.output
}