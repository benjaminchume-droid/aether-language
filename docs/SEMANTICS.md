# Aether semantic foundations

## Declarations

`let name = expression;` creates an immutable local binding. `let mut name = expression;` creates a mutable binding. Shadowing is rejected in the current bootstrap scope; explicit shadowing semantics will be introduced with the full binding model.

## Expressions

Expressions have a static type. Numeric operators accept `Int` and `Float`, promoting the result to `Float` when either operand is `Float`. `+` also concatenates two `String` values. Equality requires compatible operands. Ordering is numeric. `&&` and `||` require boolean operands.

## Functions

Functions are named items. The bootstrap checker records arity and prepares the representation used for future parameter and return type inference. Full explicit function signatures and inference constraints are added as the type system matures.

## Control flow

`if` and `while` conditions must be `Bool`. `return` may carry an expression or return `Unit`.
