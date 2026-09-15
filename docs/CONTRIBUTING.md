# Contributing to Aether Language

Every language feature should be implemented across the full contract it affects: lexer, parser, AST, semantics, execution, diagnostics, tests, and documentation where applicable.

Prefer small composable modules. Do not encode platform-specific behavior into the parser. Keep public language semantics deterministic and testable.
