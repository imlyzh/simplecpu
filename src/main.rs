use crate::memory_sub_system::{phy_mem_init, read_u8};

#[macro_use]
mod utils;
mod riscv_utils;

mod direct_interpreter;
mod memory_sub_system;



fn main() {
    println!("Hello, world!");
    phy_mem_init();
    dbg!(read_u8(10));
}
