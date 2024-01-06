use std::env;
use std::fs;
mod scanner;

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() == 1 {
        //run prompt left for now
       
    } else {
        run_file(&arg[1])
    }
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Error: file doesnt exist");
    
    let mut scanner = scanner::scan_tokens(contents);
}
