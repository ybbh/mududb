use crate::common::buf::Buf;
use crate::common::endian::Endian;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinSlot {
    off: u32,
    len: u32,
}

impl BinSlot {
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() < Self::size_of() {
            panic!("slot capacity  error");
        }
        let off = Endian::read_u32(slice);
        let len = Endian::read_u32(&slice[size_of::<u32>()..]);
        Self::new(off, len)
    }

    pub fn copy_to_slice(&self, binary: &mut [u8]) {
        if binary.len() < Self::size_of() {
            panic!("binary slot capacity  error");
        }
        Endian::write_u32(binary, self.off);
        Endian::write_u32(&mut binary[size_of::<u32>()..], self.len);
    }

    pub fn to_binary(&self) -> Buf {
        let mut buf: Buf = vec![0; Self::size_of()];
        Endian::write_u32(&mut buf, self.off);
        Endian::write_u32(&mut buf[size_of::<u32>()..], self.len);
        buf
    }

    pub fn new(off: u32, len: u32) -> Self {
        Self { off, len }
    }

    pub fn offset(&self) -> u32 {
        self.off
    }

    pub fn length(&self) -> u32 {
        self.len
    }

    pub fn size_of() -> usize {
        size_of::<u32>() + size_of::<u32>()
    }
}
