
//! Dispatch blobs

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

pub trait ZeroCopy: Sized {
    fn size(data: &mut [u8]) -> Result<u32, DecodeError>;

    fn view(data: &mut [u8]) -> &Self;

    fn copy_from(data: &mut [u8]) -> Self;

    fn copy_to(self, data: &mut [u8]);

    fn instance_size(&self) -> u32;

    fn is_fixed_size() -> bool;
}

pub trait Fixed {
    fn fixed_size() -> u32;
}

impl DrainBlob {
    pub fn new(data: Vec<u8>) -> Self {
        DrainBlob {
            data,
            position: 0,
        }
    }

    pub fn next<T: ZeroCopy>(&mut self) -> Result<&T, DecodeError> {
        let len = T::size(&mut self.data)? as usize;

        if self.position + len > self.data.len() { return Err(DecodeError::UnexpectedEof); }

        let result = T::view(&mut self.data[self.position..self.position + len]);

        self.position += len;

        Ok(result)
    }
}

impl ResultBlob {
    pub fn new() -> Self { Self { data: vec![] } }

    pub fn as_bytes(&self) -> &[u8] { &self.data }

    pub fn push<E: ZeroCopy>(&mut self, e: E) {
        let instance_size = e.instance_size();
        if !E::is_fixed_size() {
            self.push(e.instance_size());
        }

        let instance_size = instance_size as usize;
        self.data.resize(self.data.len() + instance_size, 0);
        let tail = self.data.len() - instance_size;
        e.copy_to(&mut self.data[tail..])
    }
}
