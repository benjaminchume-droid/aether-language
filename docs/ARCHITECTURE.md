# Aether Language architecture

Aether is implemented as a layered language platform. Source-level syntax is defined once and shared by interpreted, intermediate, and native execution paths.

```text
                 .ae source
                     |
                   Lexer
                     |
                   Parser
                     |
                    AST
                     |
           Semantic analysis / types
                     |
                 High-level IR
                /             \
          Interpreter       Native backends
                \             /
                 Runtime + Std
```

The compiler front-end must remain independent of target operating systems. Platform integration belongs below the language/runtime boundary.

## Runtime boundary

The runtime owns representation and execution services: values, allocation, memory regions, buffers, synchronization, tasks, asynchronous I/O, processes, filesystem, networking, FFI, and platform adapters.

## Self-hosting boundary

Rust is the bootstrap implementation. Public compiler contracts are kept deterministic and testable so those components can later be reimplemented in Aether without changing source semantics.
