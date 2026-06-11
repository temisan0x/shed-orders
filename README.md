# Dynamic Window 🍔

A small Rust project simulating a drive-thru queue. Built mostly to get hands-on with
how Rust handles memory — stack vs heap, ownership, and why you don't need a GC.

## What I was playing with:
- `String` vs fixed types — why the customer queue lives on the heap
- `mut` and how ownership + borrowing interact with mutability
- How Rust cleans up heap memory at end of scope without a garbage collector