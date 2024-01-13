use std::env;
use std::fs;
mod scanner;
mod util;

fn main() {
    let arg: Vec<String> = env::args().collect();
    if &arg[1] == "compile" {
        if arg.len() == 1 {
            //run prompt left for now
        } else {
            run_file(&arg[2])
        }
    } else if &arg[1] == "generate_ast" {
        util::generate_ast::generate_ast(&arg[2])
    }
   
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Error: file doesnt exist");
    
    let scanner = scanner::scan(contents);
    scanner.display_tokens();
}
