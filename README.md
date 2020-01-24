# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)

For examples you can see [grammars](grammars) and [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for usage 

### Implementation status

Everything is implemented, but you should still expect bugs/panics, so it is not ready for production yet.
Also API very likely will have some changes.

Currently requires nightly version of rust. 
This very likely will be the case until specialization and try blocks are stabilized. 

Remaining things:
- [ ] Parser
  - [ ] some internal optimizations
  - [ ] retrieve child by index if children have labeled alternatives
- [ ] Generator
  - [ ] CI  
- [ ] Documentation
- [ ] API stabilization
  - [ ] Rust api guidelines compliance   
  
### Dependencies

```toml 
[dependencies]
lazy_static = "1.4"
antlr-rust = "0.1"
```
  
### Differencies with Java
Although Rust runtime API is made as close as possible to Java, 
there are quite some differences because Rust is not OOP and is much more explicit. 

 - All rule context variables should implement `Default`. 
  
### Future improvements:
 - make parsing zero copy(i.e. use &str instead String in token and &Token in tree nodes)
 - use & instead of Rc for nodes in parser
 - support no_std(although alloc would still be required)
 - support stable rust

## Licence

BSD 3-clause 
