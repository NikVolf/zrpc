# zrpc
Minimal RPC for Rust

## Design goals

- Non-forcing. You just write your api, put `#[rpc]` and it just works!
- Very minimal, cnvention over configuration.
- Multi-transport
- Async/await
- Server references! Zero-copy! and much more!

## Easy to use

See `examples/minimal`

## Server references

Imagine there are two entities `A` and `B`. We invoke rpc `get_item(a_id) -> A`. We also have a method `generate_b(&A)`.

We can avoid all the data from A having to go from server to client if we invoke something like `generate_b(get_item(1))` by first returning lazy server reference from `get_item` and dereferencing by passing actual data of `A` only when required.

## Zero-copy

When you have something like this signature in your rpc:

```rust
fn apply(&mut self, raw: &[u8]);
```

you'll work directly with the buffer from network!
