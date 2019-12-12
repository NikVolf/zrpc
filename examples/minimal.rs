use zrpc_net::ipc;

struct Accumulator(u64);

#[zrpc_derive::rpc]
impl Accumulator {
	fn add(&mut self, val: &u64) -> u64 {
		self.0 += val;
		self.0
	}

	fn sub(&mut self, val: &u64) -> u64 {
		self.0 = self.0.saturating_sub(*val);
		self.0
	}

	fn set(&mut self, val: &u64) {
		self.0 = *val;
	}
}

#[tokio::main]
async fn main() {
	let path = std::env::args().nth(1).expect("Run it with server path as argument");
	ipc::run_service(path, Accumulator(0)).await;
}