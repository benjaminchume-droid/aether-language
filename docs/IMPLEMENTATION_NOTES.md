# Implementation notes

The bootstrap currently keeps the public front-end small and explicit. Internal representations are allowed to evolve behind stable module boundaries.

The next compiler work should move from primitive type checking toward real constraint generation and inference, then into richer declarations, pattern matching, error handling, ownership/borrowing, and collection semantics.
