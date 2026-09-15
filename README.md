# Aether Language

Aether (`.ae`) is the native programming language of the Aether computing ecosystem.

This repository contains the complete compiler, runtime, standard library, tooling, language services, and platform integrations required to build the language into a self-hosting systems and application platform.

## Engineering direction

Aether is built as one language with multiple depths: approachable application programming through systems, graphics, AI, networking, and OS development.

Bootstrap infrastructure is implemented in Rust. The compiler, runtime, standard library, and tooling are designed so that Aether can progressively replace its bootstrap implementation and become self-hosting.

## Repository layout

- `crates/aether`: language front-end and execution core
- `compiler/`: compiler pipeline and future native backends
- `runtime/`: execution, memory, concurrency, I/O, and platform runtime
- `std/`: Aether standard library
- `tools/`: developer tooling
- `tests/`: language and compiler conformance tests
- `examples/`: runnable `.ae` programs
- `docs/`: language and architecture documentation
