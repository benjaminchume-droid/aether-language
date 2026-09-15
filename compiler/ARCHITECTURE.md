# Compiler architecture

The bootstrap compiler is intentionally layered so each stage has one responsibility:

```text
Source
  -> Lexer
  -> Parser
  -> AST
  -> Semantic analysis / type checking
  -> High-level IR
  -> Optimization
  -> Target lowering
  -> Object / executable
```

The interpreter is an execution backend, not a second language implementation. It consumes the same parsed and checked program model. Native backends will use the same language semantics.

## Invariants

1. Parsing never performs type inference.
2. Semantic analysis never mutates the source AST into target-specific instructions.
3. Runtime behavior is defined independently from the CLI.
4. Diagnostics have stable codes and source spans.
5. Backends cannot silently change language semantics.
