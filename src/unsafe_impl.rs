
impl crate::Elementary for u64 {
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