# antlr4rust
[![Crate](https://flat.badgen.net/crates/v/antlr-rust)](https://crates.io/crates/antlr_rust/0.3.0-beta)
[![docs](https://flat.badgen.net/badge/docs.rs/v0.3.0-beta)](https://docs.rs/antlr-rust/0.3.0-beta)
![ANTLR4 testsuite](https://github.com/rrevenantt/antlr4rust/workflows/ANTLR4%20testsuite/badge.svg?event=push)
![cargo test](https://github.com/rrevenantt/antlr4rust/workflows/cargo%20test/badge.svg)
[![](https://tokei.rs/b1/github/rrevenantt/antlr4rust)](https://github.com/rrevenantt/antlr4rust)

[ANTLR4](https://github.com/antlr/antlr4) runtime for Rust programming language.

For examples you can see [grammars](grammars), [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for actual usage examples

## ANTLR4 Tool(parser generator)

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)
Latest version is automatically built to [releases](https://github.com/rrevenantt/antlr4rust/releases) on this repository.
So if you just want to generate parser 
or if you want to contribute to only runtime part you don't have to do build it yourself. 

But if you want to build or change generator yourself:
* `git clone -b rust-target https://github.com/rrevenantt/antlr4` - clone my antlr4 fork  
* `git submodule update --init --recursive` - update Rust target submodule
* `mvn -DskipTests install` - build generator

### Implementation status

For now development is going on in this repository 
but eventually it will be merged to main ANTLR4 repo

Since version `0.3` works on stable rust.
Previous versions are not maintained any more 
so in case of nightly breakage you should migrate to the latest version. 

### Usage

You should use the ANTLR4 "tool" to generate a parser, that will use the ANTLR 
runtime located here. You can run it with the following command:
```bash
java -jar <path to ANTLR4 tool> -Dlanguage=Rust MyGrammar.g4
```
For a full list of antlr4 tool options, please visit the 
[tool documentation page](https://github.com/antlr/antlr4/blob/master/doc/tool-options.md).

You can also see [build.rs](build.rs) as an example of `build.rs` configuration 
to rebuild parser automatically if grammar file was changed.

Then add following to `Cargo.toml` of the crate from which generated parser 
is going to be used:
```toml 
[dependencies]
antlr-rust = "0.3"
```
 
### Parse Tree structure

It is possible to generate idiomatic Rust syntax trees. For this you would need to use labels feature of ANTLR tool.
You can see [Labels](grammars/Labels.g4) grammar for example.
Consider following rule :
```text
e   : a=e op='*' b=e   # mult
    | left=e '+' b=e   # add
		 
```
For such rule ANTLR will generate enum `EContextAll` containing `mult` and `add` alternatives, 
so you will be able to match on them in your code. 
Also corresponding struct for each alternative will contain fields you labeled. 
I.e. for `MultContext` struct will contain `a` and `b` fields containing child subtrees and 
`op` field with `TerminalNode` type which corresponds to individual `Token`.
It also is possible to disable generic parse tree creation to keep only selected children via
`parser.build_parse_trees = false`, but unfortunately currently it will prevent visitors from working. 
  
### Differences with Java
Although Rust runtime API has been made as close as possible to Java, 
there are quite some differences because Rust is not an OOP language and is much more explicit. 

 - If you are using labeled alternatives, 
 struct generated for the rule is an enum with variant for each alternative
 - Parser needs to have ownership for listeners, but it is possible to get listener back via `ListenerId`
 otherwise `ParseTreeWalker` should be used.
 - In embedded actions to access parser you should use `recog` variable instead of `self`/`this`. 
 This is because predicates have to be inserted into two syntactically different places in generated parser 
 and in one of them it is impossible to have parser as `self`.
 - str based `InputStream` have different index behavior when there are unicode characters. 
 If you need exactly the same behavior, use `[u32]` based `InputStream`, or implement custom `CharStream`.
 - In actions you have to escape `'` in rust lifetimes with `\ ` because ANTLR considers them as strings, e.g. `Struct<\'lifetime>`
 - To make custom tokens you should use `@tokenfactory` custom action, instead of usual `TokenLabelType` parser option.
 ANTLR parser options can accept only single identifiers while Rust target needs know about lifetime as well. 
 Also in Rust target `TokenFactory` is the way to specify token type. As example you can see [CSV](grammars/CSV.g4) test grammar.
 - All rule context variables (rule argument or rule return) should implement `Default + Clone`.
 
### Benchmarks
Here is comparison of antlr generated XML lexer and parser
(from default XML grammar but with custom minimal Token/TokenFactory/InputStream/RuleContext) to hand-written implementations in rust ecosystem.
Keep in mind that `xmlparser` and `quick_xml` are much closer to being lexer than parser, so they should be compared with antlr lexer.
Also while structs used by generated lexer and parser were customized to track as minimum data as required 
(which is possible by any user of antlr-rust), 
internals of the lexer cannot be customized enough yet and still track quite a lot of data that might not be used in particular case. 
So there is still room for improvement.
```text
lexers:
large/large_xmlparser        time:   [1.8598 ms 1.8607 ms 1.8619 ms]                                   
large/large_quick_xml        time:   [1.4623 ms 1.4645 ms 1.4675 ms]                                   
large/large_antlr_xml_lexer  time:   [5.7866 ms 5.7877 ms 5.7891 ms]
parsers:
large/large_xmlrs            time:   [16.734 ms 16.748 ms 16.766 ms]
large/large_minidom          time:   [7.0639 ms 7.0792 ms 7.0975 ms]                                
large/large_roxmltree        time:   [4.9341 ms 4.9360 ms 4.9380 ms]                                   
large/large_antlr_xml_full   time:   [10.243 ms 10.248 ms 10.252 ms]                                  
```

### Unsafe
Currently, unsafe is used only for downcasting (through separate crate) 
and to update data inside Rc via `get_mut_unchecked`(returned mutable reference is used immediately and not stored anywhere)

### Versioning
In addition to usual Rust semantic versioning, 
patch version changes of the crate should not require updating of generator part 
  
## Licence

BSD 3-clause. 
Unless you explicitly state otherwise, 
any contribution intentionally submitted for inclusion in this project by you
shall be licensed as above, without any additional terms or conditions.

