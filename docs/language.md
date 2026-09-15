# Aether Language

## Core semantics

Aether is expression-oriented. Statements that produce values may be used in expression position where the grammar permits it. Variables are immutable by default; `let mut` opts into mutation.

## Primitive types

- `Int`: signed machine integer
- `Float`: IEEE-754 double precision
- `Bool`: boolean
- `String`: UTF-8 string
- `Unit`: absence of a meaningful value

The front-end will grow toward explicit, statically checked type syntax while retaining inference for ordinary local variables.

## Equality

`==` and `!=` compare runtime values. Numeric equality permits `Int`/`Float` comparison after numeric promotion.

## Diagnostics

Compiler errors should carry source spans and a stable diagnostic code so the CLI, LSP, and future IDE tooling can consume them consistently.
