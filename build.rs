//! Used to generate tests.
//! Not uploaded to crates.io, so this build script should not affect users if this crate is used as a regular crates.io dependency
//!
//! set ANTLR4_PATH=download if you don't want/need a full rebuild of antlr tool,
//! and it will download a corresponding artifact right from github releases
use std::env;
use std::env::VarError;
use std::error::Error;
use std::path::Path;
use std::process::Command;

fn main() {
    let grammars = vec![
        "VisitorBasic",
        "VisitorCalc",
        "CSV",
        "ReferenceToATN",
        "XMLLexer",
        "SimpleLR",
        "Labels",
    ];
    let additional_args = vec![
        Some("-visitor"),
        Some("-visitor"),
        Some("-visitor"),
        None,
        None,
        None,
        None,
    ];

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ANTLR4_PATH");

    let antlr_path = match env::var("ANTLR4_PATH").as_deref() {
        Ok("download") => {
            let url = format!(
                "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust{}/antlr4-4.8-2-SNAPSHOT-complete.jar",
                env::var("CARGO_PKG_VERSION").unwrap()
            );
            let path = Path::new(&env::var("OUT_DIR").unwrap()).join("tool.jar");
            Command::new("curl")
                .arg(url)
                .arg("-L")
                .arg("-o")
                .arg(&path)
                .spawn()
                .unwrap()
                .wait_with_output()
                .expect("error running curl");
            path.to_str().unwrap().to_string()
        }
        Ok("skip") => return,
        Ok(x) => Path::new(x)
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        Err(VarError::NotUnicode(_)) => panic!("non unicode env variable"),
        Err(_) => {
            let default_path = env::current_dir()
                .unwrap()
                .join("../../tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar")
                .canonicalize();
            match default_path {
                Ok(x) if !x.exists() => return,
                Err(_) => return,
                Ok(x) => x.to_str().unwrap().to_string(),
            }
        }
    };

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        gen_for_grammar(grammar, &antlr_path, arg).expect("error running antlr tool");
    }

    println!("cargo:rerun-if-changed={}", antlr_path);
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let input = env::current_dir().unwrap().join("grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("../tests/gen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()?
        .wait_with_output()?;

    println!("cargo:rerun-if-changed=grammars/{}", file_name);
    Ok(())
}
