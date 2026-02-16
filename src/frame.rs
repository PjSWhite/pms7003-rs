use super::PmsU16Int;
use zerocopy::{FromBytes, Immutable, IntoBytes, Unaligned};

#[repr(C)]
#[derive(FromBytes, Unaligned)]
pub struct Pms7003DataFrame {
    magic: PmsU16Int,
    len: PmsU16Int,
    pub pm1_0_std: PmsU16Int,
    pub pm2_5_std: PmsU16Int,
    pub pm10_std: PmsU16Int,

    pub pm1_0_atm: PmsU16Int,
    pub pm2_5_atm: PmsU16Int,
    pub pm10_atm: PmsU16Int,

    pub pc_0_3um: PmsU16Int,
    pub pc_0_5um: PmsU16Int,
    pub pc_1_0um: PmsU16Int,
    pub pc_2_5um: PmsU16Int,
    pub pc_5_0um: PmsU16Int,
    pub pc_10um: PmsU16Int,

    _reserved: PmsU16Int,
    check_code: PmsU16Int,
}

#[repr(C, packed)]
#[derive(IntoBytes, Unaligned, Immutable, Clone, Copy)]
pub(super) struct Pms7003CommandFrame {
    pub(super) magic: PmsU16Int,
    pub(super) cmd: u8,
    pub(super) data: PmsU16Int,
    pub(super) check_code: PmsU16Int,
}
