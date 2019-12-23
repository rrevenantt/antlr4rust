# antlr4rust
ANTLR4 runtime for Rust programming language 

Generator part is currently located in [rrevenantt/antlr4](https://github.com/rrevenantt/antlr4)

For examples you can see [grammars](grammars) and [tests/gen](tests/gen) for corresponding generated code 

# Implementation status

WIP, not quite useful yet

Missing features:
- [ ] Lexer
  - [ ] Some corner cases from ANTLR4 test suit are still failing
- [ ] Parser
  - [ ] recovery is partially working
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
