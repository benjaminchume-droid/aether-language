# Aether Language implementation order

The project is built completely, in dependency order. A milestone is not a reduced product; it is a stable slice of the final architecture.

1. Language front-end: tokens, parser, AST, diagnostics, source spans, type system, semantic analysis.
2. Language semantics: modules, functions, closures, collections, structs, enums, pattern matching, errors, generics, traits, ownership, borrowing, lifetimes.
3. Execution: bytecode/IR, interpreter, runtime, allocator, buffers, I/O, filesystem, processes, FFI.
4. Concurrency: threads, tasks, async/await, channels, synchronization, cancellation.
5. Native compiler: SSA-like IR, optimization, target abstraction, ABI, object generation, linker integration, debug information.
6. Standard library and tooling: package manager, formatter, linter, test runner, debugger, profiler, LSP, documentation.
7. Platform domains: networking, HTTP, graphics, GPU, UI, multimedia, database APIs, AI/tensor primitives, web targets.
8. Self-hosting: progressively port compiler/runtime/tooling components to Aether.

No stage removes earlier functionality. New stages extend the same language and compatibility contract.
