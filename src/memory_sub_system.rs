use std::mem::MaybeUninit;




static mut PHY_LENGTH: usize = 0;
static mut PHY_MEMORY: *mut u8 = core::ptr::null_mut();

pub fn phy_mem_init() {
  unsafe {
    let vec = Vec::with_capacity(128*1024*1024);
    PHY_LENGTH = 128*1024*1024;
    PHY_MEMORY = vec.leak().as_mut_ptr();
  }
}

#[inline(always)]
pub fn read_u8(offset: usize) -> Option<u8> {
  unsafe { Some(*PHY_MEMORY.add(offset)) }
}

#[inline(always)]
pub fn read_u16(offset: usize) -> Option<u16> {
  let mut r = read_u8(offset)? as u16;
  let l = read_u8(offset+1)? as u16;
  r |= l << 8;
  Some(r)
}

#[inline(always)]
pub fn read_u32(offset: usize) -> Option<u32> {
  let mut r = read_u16(offset)? as u32;
  let l = read_u16(offset+2)? as u32;
  r |= l << 16;
  Some(r)
}

#[inline(always)]
pub fn read_u64(offset: usize) -> Option<u64> {
  let mut r = read_u16(offset)? as u64;
  let l = read_u16(offset+4)? as u64;
  r |= l << 32;
  Some(r)
}

#[inline(always)]
pub fn write_u8(offset: usize, value: u8) -> Option<()> {
  todo!()
}

#[inline(always)]
pub fn write_u16(offset: usize, value: u16) -> Option<()> {
  todo!()
}

#[inline(always)]
pub fn write_u32(offset: usize, value: u32) -> Option<()> {
  todo!()
}

#[inline(always)]
pub fn write_u64(offset: usize, value: u64) -> Option<()> {
  todo!()
}

#[inline(always)]
pub fn write_u128(offset: usize, value: u128) -> Option<()> {
  todo!()
}
