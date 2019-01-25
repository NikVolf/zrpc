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