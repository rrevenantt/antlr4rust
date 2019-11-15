use std::env;
use std::fs::{File, DirEntry, read_dir};
use std::io::Write;
use std::path::Path;
use std::env::VarError;
use std::process::Command;


fn main() {
    let grammars = vec!["CSV", "ReferenceToATN", "XMLLexer"];
//    let antlr_path = match env::var("ANTLR4_GENERATOR"){
//        Ok(x) => x,
//        Err(_) => return,
//    };
    let antlr_path = "/home/rrevenantt/dev/antlr4/tool/target/antlr4-4.7.2-SNAPSHOT-complete.jar";

    for grammar in grammars {
        gen_for_grammar(grammar, antlr_path);
    }

    for file in read_dir(env::var("OUT_DIR").unwrap()).unwrap() {
        if let Ok(file) = file {
            if !file.file_name().to_str().unwrap().ends_with(".rs") { continue; }
        }
    }
}

fn gen_for_grammar(grammar_file_name: &str, antlr_path: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);

    let input = env::current_dir().unwrap().join("grammars").join(grammar_file_name.to_owned() + ".g4");

//    let mut f = File::create(&dest_path).unwrap();


    Command::new("java")
        .args(&["-Xmx500M", "-cp", antlr_path,
            "org.antlr.v4.Tool", "-Dlanguage=Rust", "-o", &out_dir, input.to_str().unwrap()])
        .status().unwrap();
}