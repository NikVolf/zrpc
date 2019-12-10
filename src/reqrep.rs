use crate::blob::{DrainBlob, ResultBlob};
use futures::Future;

pub trait Service {
    type MethodId;
    type Future: Future<Output=ResultBlob> + Send;

    fn handle(&mut self, method: Self::MethodId, arguments: DrainBlob) -> Self::Future;
}