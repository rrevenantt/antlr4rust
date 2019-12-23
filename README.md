# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in rust-target branch of my antlr4 fork [rrevenantt/antlr4/tree/rust-target](https://github.com/rrevenantt/antlr4/tree/rust-target)

For examples you can see [grammars](grammars) and [tests/gen](tests/gen) for corresponding generated code 
and [tests/my_tests.rs](tests/my_test.rs) for usage 

# Implementation status

WIP, some basic things are working but still not quite useful yet

Missing features(everything else is pretty much implemented, but you should still expect bugs/panics):
- [ ] Lexer
  - [ ] Some corner cases from ANTLR4 test suit are still failing
- [ ] Parser
  - [ ] recovery/error reporting is partially working
  - [ ] some internal optimizations
  - [ ] labeled alternatives
  - [ ] left recursion
  - [ ] syntax tree
  - [ ] Full testing with ANTLR4 test suit(currently about 50% parser logic tests are passing )
- [ ] Generator
  - [ ] Rebase to upstream
  - [ ] CI
- [ ] Documentation
- [ ] API stabilization 

# Licence

MIT 
