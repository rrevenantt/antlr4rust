fn main() {
    #[cfg(feature = "generate_test_grammars")]
    {
        self::antlr_tool::generate_grammars();
    }
}

#[cfg(feature = "generate_test_grammars")]
mod antlr_tool {
    use std::{env, process::Command};

    const ANTLR_PATH: &str = "ANTLR_PATH";

    const GRAMMARS_AND_ADDITION_ARGS: &[(&str, Option<&str>)] = &[
        ("CSV", Some("-visitor")),
        ("Labels", None),
        ("Perf", None),
        ("ReferenceToATN", None),
        ("SimpleLR", None),
        ("XMLLexer", None),
    ];

    fn antlr_path_env() -> String {
        env::var(ANTLR_PATH).expect("Missing ANTLR_PATH environmental variable")
    }

    pub fn generate_grammars() {
        for (grammar, arg) in GRAMMARS_AND_ADDITION_ARGS {
            gen_for_grammar(grammar, *arg);
        }

        println!("cargo:rerun-if-changed=build.rs");

        println!("cargo:rerun-if-env-changed={}", ANTLR_PATH);

        println!("cargo:rerun-if-changed={}", antlr_path_env());
    }

    fn gen_for_grammar(grammar_file_name: &str, additional_arg: Option<&str>) {
        let input = env::current_dir()
            .expect("Could not retrieve current directory")
            .join("grammars");
        let file_name = grammar_file_name.to_owned() + ".g4";

        let exit_status = Command::new("java")
            .current_dir(input)
            .arg("-cp")
            .arg(antlr_path_env())
            .arg("org.antlr.v4.Tool")
            .arg("-Dlanguage=Rust")
            .arg("-o")
            .arg("../tests/gen")
            .arg(&file_name)
            .args(additional_arg)
            .spawn()
            .expect("antlr tool failed to start")
            .wait()
            .expect("antlr tool wasn't running");

        assert!(
            exit_status.success(),
            "antlr tool failed to generate grammar"
        );

        let exit_status = Command::new("cargo")
            .arg("fmt")
            .spawn()
            .expect("`cargo fmt` failed")
            .wait()
            .expect("rustfmt wasn't running");

        assert!(exit_status.success(), "rustfmt failed to format");

        println!("cargo:rerun-if-changed=grammars/{}", file_name);
    }
}
