struct Dummy;

#[zrpc_derive::rpc]
impl Dummy {
    fn membuf(&mut self, a: &[u8]) {
    }
}

fn main() {

}