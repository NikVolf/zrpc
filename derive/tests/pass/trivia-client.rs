struct Dummy;

#[zrpc_derive::rpc(client)]
impl Dummy {
    fn membuf(&mut self, a: &[u8]) -> u32 {
        a.len() as u32
    }
}

fn main() {
}