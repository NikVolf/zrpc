use crate::{ZeroCopy, PushValue, DecodeError};

impl ZeroCopy for &u64 {

    // Fixed size for u64
    fn size(_data: &mut [u8]) -> Result<u32, DecodeError> { Ok(8) }

    fn view(data: &mut [u8]) -> Self {
        // data.len is guaranteed to be at least size() for elementary parameters
        unsafe { std::mem::transmute::<*const u8, &u64>(data.as_ptr()) }

        // TODO: if endianess is not le, view() actually reshuffles in place!
    }

}

impl PushValue for u64 {

    fn copy_from(data: &mut[u8]) -> Self {
        let mut dat = [0u8; 8];
        dat.copy_from_slice(&data);
        Self::from_le_bytes(dat)
    }

    fn copy_to(self, buf: &mut [u8]) {
        // buf.len is guaranteed to be at least size() for elementary parameters
        buf.copy_from_slice(&self.to_le_bytes());
    }

    fn instance_size(&self) -> u32 {
        8
    }

    fn is_fixed_size() -> bool {
        true
    }
}

impl ZeroCopy for &u32 {

    // Fixed size for u64
    fn size(_data: &mut [u8]) -> Result<u32, DecodeError> { Ok(8) }

    fn view(data: &mut [u8]) -> Self {
        // data.len is guaranteed to be at least size() for elementary parameters
        unsafe { std::mem::transmute::<*const u8, &u32>(data.as_ptr()) }

        // TODO: if endianess is not le, view() actually reshuffles in place!
    }

}

impl PushValue for u32 {

    fn copy_from(data: &mut[u8]) -> Self {
        let mut dat = [0u8; 4];
        dat.copy_from_slice(&data);
        Self::from_le_bytes(dat)
    }

    fn copy_to(self, buf: &mut [u8]) {
        // buf.len is guaranteed to be at least size() for elementary parameters
        buf.copy_from_slice(&self.to_le_bytes());
    }

    fn instance_size(&self) -> u32 {
        4
    }

    fn is_fixed_size() -> bool {
        true
    }
}
