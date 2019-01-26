# zrpc
Minimal RPC for Rust

## Design goals

- User can abstract interaction of parts of his program so that he does not care if they run on the same machine or same binary or even distributed.
- Very minimal
- Transport agnostic
- Tokio based
- A lot of options of generating actual code from traits
- Not avoiding global state (so that communication could be efficiently hypervised!)
- Server references! Zero-copy! other fancy stuff.

## Server references

Imagine there are two entities `A` and `B`. We invoke rpc `get_item(a_id) -> A`. We also have a method `generate_b(&A)`.

We can avoid all the data from A having to go from server to client if we invoke something like `generate_b(get_item(1))` by first returning lazy server reference from `get_item` and dereferencing by passing actual data of `A` only when required.