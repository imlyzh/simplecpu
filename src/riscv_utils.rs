/// riscv inst mode

#[macro_use]
use super::utils;


/////////////////////////////
/// part of inst

macro_rules! opcode {
  ($i:expr) => {
    bits!($i, 6, 0) as u8
  };
}

macro_rules! rd {
    ($i:expr) => {
      bits!($i, 11, 7) as u8
    };
}

macro_rules! rs1 {
  ($i:expr) => {
    bits!($i, 19, 15) as u8
  };
}

macro_rules! rs2 {
  ($i:expr) => {
    bits!($i, 24, 20) as u8
  };
}

macro_rules! funct3 {
  ($i:expr) => {
    bits!($i, 14, 12) as u8
  };
}

macro_rules! funct7 {
  ($i:expr) => {
    bits!($i, 31, 25) as u8
  };
}

macro_rules! iimm {
  ($i:expr) => {
    bits!($i, 31, 20) as u32
  };
}

macro_rules! simm {
  ($i:expr) => {
    ((bits!($i, 31, 25) << 5) | bits!($i, 11, 7)) as u32
  };
}

macro_rules! uimm {
  ($i:expr) => {
    bits!($i, 31, 12) as u32
  };
}

macro_rules! bimm {
  ($i:expr) => {{
    let imm11 = bits!($i, 7, 7) as u32;
    let imm4_1 = bits!($i, 11, 8) as u32;
    let imm10_5 = bits!($i, 30, 25) as u32;
    let imm12 = bits!($i, 31, 31) as u32;
    imm4_1 << 1 | imm10_5 << 5 | imm11 << 11 | imm12 << 12
  }
  };
}

macro_rules! jimm {
  ($i:expr) => {{
    let imm19_12 = bits!($i, 19, 12) as u32;
    let imm11 = bits!($i, 20, 20) as u32;
    let imm10_1 = bits!($i, 30, 21) as u32;
    let imm20 = bits!($i, 31, 31) as u32;
    imm10_1 << 1 | imm11 << 11 | imm19_12 << 12 | imm20 << 20
  }
  };
}

//////////////////////////////////////////
/// inst type get params

macro_rules! rtype {
  ($rd:ident, $rs1:ident, $rs2:ident, $src:expr) => {
    let $rd = rd!($src);
    let $rs1 = rs1!($src);
    let $rs2 = rs2!($src);
  };
}

macro_rules! itype {
  ($rd:ident, $rs1:ident, $imm:ident, $src:expr) => {
    let $rd = rd!($src);
    let $rs1 = rs1!($src);
    let $imm = sext!(iimm!($src), 12, 32);
  };
}

macro_rules! stype {
  ($rd:ident, $rs1:ident, $rs2:ident, $imm:ident, $src:expr) => {
    let $rd = rd!($src);
    let $rs1 = rs1!($src);
    let $rs2 = rs2!($src);
    let $imm = sext!(simm!($src), 12, 32);
  };
}

macro_rules! utype {
  ($rd:ident, $imm:ident, $src:expr) => {
    let $rd = rd!($src);
    let $imm = uimm!($src) << 12;
  };
}

macro_rules! btype {
  ($rs1:ident, $rs2:ident, $branch:ident, $src:expr) => {
    let $rs1 = rs1!($src);
    let $rs2 = rs2!($src);
    let $branch = sext!(bimm!($src), 12, 32);
  };
}

macro_rules! jtype {
  ($rd:ident, $branch:ident, $src:expr) => {
    let $rd = rd!($src);
    let $branch = sext!(jimm!($src), 20, 32);
  };
}

/*
/// rtype(x) -> (rd, rs1, rs2)
#[inline(always)]
pub fn rtype(i: u32) -> (u8, u8, u8) {
  (rd!(i), rs1!(i), rs2!(i))
}

/// itype(x) -> (rd, rs1, raw imm)
#[inline(always)]
pub fn itype(i: u32) -> (u8, u8, u32) {
  (rd!(i), rs1!(i), sext!(iimm!(i), 12, 32))
}

/// stype(x) -> (rd, rs1, rs2, raw imm)
#[inline(always)]
pub fn stype(i: u32) -> (u8, u8, u8, u32) {
  (rd!(i), rs1!(i), rs2!(i), sext!(simm!(i), 12, 32))
}

/// utype(x) -> (rd, raw imm)
#[inline(always)]
pub fn utype(i: u32) -> (u8, u32) {
  (rd!(i), uimm!(i) << 12)
}

/// btype(x) -> (rs1, rs2, branch)
#[inline(always)]
pub fn btype(i: u32) -> (u8, u8, u32) {
  (rs1!(i), rs1!(i), sext!(bimm!(i), 12, 32))
}

/// jtype(x) -> (rd, branch)
#[inline(always)]
pub fn jtype(i: u32) -> (u8, u32) {
  (rd!(i), sext!(jimm!(i), 20, 32))
}
// */



#[cfg(test)]
mod tests {

  #[test]
  fn test_r_type() {
    use super::rtype;

    let src: u32 = 0b00000000_00000000_00000011_01011011;
    let (rd, rs1, rs2) = rtype(src);
    assert_eq!(rd, 0b110);
    rtype_!(rd, __, __, src);
    assert_eq!(rd, 0b110);
    // rtype_!()
  }
}