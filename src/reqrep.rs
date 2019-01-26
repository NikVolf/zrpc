use std::sync::RwLock;
use futures::Future;

trait Service {
	type Error;
	type HandleFuture: Future<Item = Vec<u8>, Error=Self::Error>;

	fn handle(&mut self, client_id: usize, data: &[u8]) -> Self::HandleFuture;
}

trait Client {
	type Error;
	type ReplyFuture: Future<Item = Vec<u8>, Error=Self::Error>;

	fn request(&mut self, data: &[u8]) -> Self::ReplyFuture;
}

struct ServiceBinding<E, F: Future<Item=Vec<u8>, Error=E>> {
	service: RwLock<Service<Error=E, HandleFuture=F>>,
}