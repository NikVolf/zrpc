struct Dummy;

#[zrpc_derive::rpc]
impl Dummy {
    fn add(&mut self, a: &u64) {
    }

    fn noop(&self) -> u64 {
        0
    }
}

fn main() {

}