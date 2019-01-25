use futures::Future;

trait Service {
	type Error;
	type HandleFuture: Future<Item = Vec<u8>, Error=Self::Error>;

	fn handle(&mut self, client_id: usize, data: &[u8]) -> Self::HandleFuture;
}

trait Client {
	type Error;
	type ReplyFuture: Future<Item = Vec<u8>, Error=Self::Error>;

	fn request(&self, data: &[u8]) -> Self::ReplyFuture;
}