

/// Abstraction for zero-copy serde
pub struct DrainBlob {
    data: Vec<u8>,
    position: usize,
}

/// Stub for response storing/referencing
pub struct ResultBlob {
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum DecodeError { UnexpectedEof, InvalidMethod }

pub trait Elementary: Sized {

    fn size() -> usize;

    fn view(data: &mut [u8]) -> &Self;

    fn copy(self, data: &mut [u8]);

}

impl DrainBlob {
    pub fn new(data: Vec<u8>) -> Self {
        DrainBlob {
            data,
            position: 0,
        }
    }

    pub fn next<T: Elementary>(&mut self) -> Result<&T, DecodeError> {
        if self.position + T::size() > self.data.len() { return Err(DecodeError::UnexpectedEof); }

        let result = T::view(&mut self.data[self.position..self.position + T::size()]);

        self.position += T::size();

        Ok(result)
    }
}

impl ResultBlob {
    pub fn new() -> Self { Self { data: vec![] } }

    pub fn as_bytes(&self) -> &[u8] { &self.data }

    pub fn push<E: Elementary>(&mut self, e: E) {
        self.data.resize(self.data.len() + E::size(), 0);
        let tail = self.data.len() - E::size();
        e.copy(&mut self.data[tail..])
    }
}

impl Elementary for u64 {

    fn size() -> usize { 8 }

    fn view(data: &mut [u8]) -> &u64 {
        // data.len is guaranteed to be at least size() for elementary parameters
        unsafe { std::mem::transmute::<*const u8, &u64>(data.as_ptr()) }

        // TODO: if endianess is not le, view() actually reshuffles in place!
    }

    fn copy(self, buf: &mut [u8]) {
        // buf.len is guaranteed to be at least size() for elementary parameters
        buf.copy_from_slice(&self.to_le_bytes());
    }

}