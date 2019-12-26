struct Dummy;

#[zrpc_derive::rpc(client)]
impl Dummy {
    fn membuf(&mut self, a: &[u8]) {
    }
}

fn main() {

}