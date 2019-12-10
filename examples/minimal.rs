use zrpc::{ReqRepService, DrainBlob, ResultBlob};
use futures::{future, StreamExt};
use tokio::{
	prelude::*,
	self,
	io::split,
};
use parity_tokio_ipc::{Endpoint, SecurityAttributes};
use std::sync::{Arc, RwLock};

struct Accumulator(u64);

impl ReqRepService for Accumulator {
    type MethodId = u16;
	type Future = future::Ready<ResultBlob>;

    fn handle(&mut self, method: Self::MethodId, mut arguments: DrainBlob) -> future::Ready<ResultBlob>
	{
		if method == 1 {
			// add
			self.0 = self.0 + arguments.next().expect("crash");
		} else if method == 2 {
			self.0 = self.0.saturating_sub(*arguments.next().expect("crash"));
		}

		let mut result = ResultBlob::new();
		result.push(self.0);
		future::ready(result)
    }
}

async fn run_service<S: ReqRepService>(path: String, s: S)
where S::MethodId: From<u16>,
	S: Send + Sync + 'static
{
	let service = Arc::new(RwLock::new(s));

	let mut endpoint = Endpoint::new(path);
	endpoint.set_security_attributes(SecurityAttributes::allow_everyone_create().unwrap());
	let mut incoming = endpoint.incoming().expect("failed to open new socket");

	while let Some(result) = incoming.next().await
	{
		match result {
			Ok(stream) => {
				let (mut reader, mut writer) = split(stream);
				let service_ptr_clone = service.clone();

				tokio::spawn(async move {
					loop {
						let mut buf = [0u8; 2];
						if let Err(_) = reader.read_exact(&mut buf).await {
							println!("Closing req-rep client socket");
							break;
						}

                        let mut blob_len_buf = [0u8; 4];
                        if let Err(_) = reader.read_exact(&mut blob_len_buf).await {
							println!("Closing req-rep client socket");
							break;
						}

                        let mut blob = vec![0u8; u32::from_le_bytes(blob_len_buf) as usize];

						if let Err(_) = reader.read_exact(&mut blob).await {
							println!("Closing req-rep client socket");
							break;
						}

                        let result_blob: S::Future = service_ptr_clone
							.write().unwrap()
							.handle(u16::from_le_bytes(buf).into(), DrainBlob::new(blob));

						if let Err(e) = writer.write_all(result_blob.await.as_bytes()).await {
							println!("Error sending response: {:?}", e);
						}

					}
				});
			},
			Err(e) => { println!("Error establising connection: {:?}", e); },
		}
	};
}

#[tokio::main]
async fn main() {
	let path = std::env::args().nth(1).expect("Run it with server path as argument");
	run_service(path, Accumulator(0)).await;
}