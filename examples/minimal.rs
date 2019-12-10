use zrpc::{ReqRepService, DrainBlob, ResultBlob, DecodeError};
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
	type Future = future::Ready<std::io::Result<ResultBlob>>;

    fn handle(&mut self, method: Self::MethodId, mut arguments: DrainBlob) -> Self::Future
	{
		let mut result = ResultBlob::new();
		match arguments.next()
			.and_then(|v| {
				if method == 1 {
					// add
					Ok(self.0 + v)
				} else if method == 2 {
					// sub
					Ok(self.0.saturating_sub(*v))
				} else {
					Err(DecodeError::InvalidMethod)
				}
			})
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))
		{
			Ok(v) => {
				self.0 = v;
				result.push(self.0);
				future::ready(Ok(result))
			},
			Err(e) => {
				future::ready(Err(e))
			}
		}
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

						if let Err(_) = reader.read_exact(&mut blob[..]).await {
							println!("Closing req-rep client socket");
							break;
						}

                        let result_blob_future: S::Future = service_ptr_clone
							.write().unwrap()
							.handle(u16::from_le_bytes(buf).into(), DrainBlob::new(blob));

						let result_blob: ResultBlob = match result_blob_future.await {
							Ok(blob) => blob,
							Err(e) => {
								println!("Error handling request: {:?}", e);

								if let Err(e) = writer.write_all(&[0u8; 4]).await {
									println!("Error sending response: {:?}... closing socket", e);
									break;
								}

								continue;
							}
						};

						if let Err(e) = writer.write_all(&(result_blob.as_bytes().len() as u32).to_le_bytes()).await {
							println!("Error sending response: {:?}... closing socket", e);
							break;
						}

						if let Err(e) = writer.write_all(result_blob.as_bytes()).await {
							println!("Error sending response: {:?}... closing socket", e);
							break;
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