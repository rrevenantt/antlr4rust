# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)

For examples you can see [grammars](grammars) and [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for usage 

# Implementation status

WIP, almost all tests are passing, but you should still expect bugs/panics.
You might find it useful already, but it is not ready for production yet.
Also API very likely will have some changes.

Currently requires nightly version of rust. 
This very likely will be the case until specialization is stabilized. 

Remaining things:
- [ ] Parser
  - [ ] some internal optimizations
  - [ ] retrieve child by index if children have labeled alternatives
- [ ] Generator
  - [ ] CI
- [ ] Documentation
- [ ] API stabilization
  - [ ] Rust api guidelines compliance   
  
#    
  
# Future improvements:
 - make parsing zero copy(i.e. use &str instead String in token and &Token in tree nodes)
 - use & instead of Rc for nodes in parser
 - support no_std(although alloc would still be required)
 - support stable rust

# Licence

BSD 3-clause 
