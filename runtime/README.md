# Aether runtime

The runtime owns execution services that are shared by compiled Aether programs: memory allocation, buffers, tasks, asynchronous I/O, synchronization, process services, filesystem access, networking, and FFI.

Runtime APIs are kept separate from the compiler so native code generation and interpreted execution can share the same semantic contract.
