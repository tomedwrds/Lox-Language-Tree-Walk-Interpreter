use std::{fs::{File, self}, fmt::Error, io::Write, fmt::Display};

pub fn generate_ast(out_dir: &str) {
    let ast_generator: ASTGenerator = ASTGenerator { 
        out_dir: String::from(out_dir)
    };
    ast_generator.define_ast("a", Vec::new())
}

struct ASTGenerator {
    out_dir: String
}


impl ASTGenerator {
    fn define_ast(self, base_name: &str, types: Vec<&str>) {
        //let file_path = "src/" + self.out_dir + "/" + base_name + ".rs";
        //print!("{:?}",file_path);
        fs::create_dir("src/a");
        let file = File::create("src/a/AST.rs");
        print!("{:?}",file);
        if let Ok(mut file) = file {
            file.write(b"test");
        } else {
            print!("Error")
        }

    }
}
