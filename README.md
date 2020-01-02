# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)

For examples you can see [grammars](grammars) and [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for usage 

# Implementation status

WIP, most of the logic is working(almost all antls test suit tests related to parsing/lexing logic are passing) but you should still expect bugs/panics.
If you are not going to use yet unimplemented features you might find it useful already.

Missing features:
- [ ] Lexer
  - [ ] Couple corner cases from ANTLR4 test suit are still failing
- [ ] Parser
  - [ ] recovery/error reporting is partially working
  - [ ] some internal optimizations
  - [ ] labeled alternatives/childs
  - [ ] return values
  - [ ] Full testing with ANTLR4 test suit(currently about 95% parser logic tests are passing )
- [ ] Generator
  - [ ] Rebase to upstream
  - [ ] CI
- [ ] Documentation
- [ ] API stabilization
  - [ ] Rust api guidelines compliance   

# Licence

MIT 
