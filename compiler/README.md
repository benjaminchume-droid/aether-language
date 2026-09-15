# Aether compiler

The compiler is structured as a pipeline:

`source -> lexer -> parser -> AST -> type checker -> IR -> optimizer -> backend -> executable`

The current Rust bootstrap owns the front-end and typed representation. Later stages are added without changing the source-language contract.
