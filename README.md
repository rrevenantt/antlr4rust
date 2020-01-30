# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)

For examples you can see [grammars](grammars), [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for actual usage examples 

### Implementation status

Everything is implemented, but you should still expect bugs/panics, so it is not ready for production yet.
Also API very likely will have some changes.

Currently requires nightly version of rust. 
This very likely will be the case until `specialization`,`try_blocks` and `unsize` features are stabilized. 

Remaining core things:
- [ ] Performance
  - [ ] Some redundant cloning is happening inside which can result in performance degradation in some corner cases for left recursive rules
- [ ] Documentation
  - [ ] Quite some things are already documented but  
- [ ] API stabilization
  - [ ] Rust api guidelines compliance   
  
### Usage

Add to `Cargo.toml`
```toml 
[dependencies]
lazy_static = "1.4"
antlr-rust = "0.1"
```
and `#![feature(try_blocks)]` in your project root module
  
### Differences with Java
Although Rust runtime API is made as close as possible to Java, 
there are quite some differences because Rust is not an OOP language and is much more explicit. 

 - All rule context variables (rule argument or rule return) should implement `Default + Clone`.
 - If you are using labeled alternatives, 
 struct generated for rule is a enum with variant for each alternative
 - Parser needs to have ownership for listeners, but it is possible te get listener back via `ListenerId`
 or `ParseTreeWalker` should be used 
 
### Unsafe
Currently unsafe is used only to cast from trait object back to original type 
and to update data inside Rc via `get_mut_unchecked`(returned mutable reference is used immediately and not stored anywhere)

  
### Future improvements:
 - make parsing zero copy(i.e. use &str(or Cow) instead String in token and &Token in tree nodes)
 - use & instead of Rc for nodes in parser
 - support no_std(although alloc would still be required)
 - support stable rust
 - visitor

## Licence

BSD 3-clause 
