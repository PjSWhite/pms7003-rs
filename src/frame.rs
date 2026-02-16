use super::PmsU16Int;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

const PMS7003MAGIC: PmsU16Int = PmsU16Int::new(0x424d);

#[repr(C)]
#[derive(FromBytes, Unaligned, Immutable, KnownLayout)]
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
    magic: PmsU16Int,
    cmd: u8,
    data: PmsU16Int,
    check_code: PmsU16Int,
}

impl Pms7003CommandFrame {
    pub fn new(cmd: u8, data: PmsU16Int) -> Self {
        Self {
            magic: PMS7003MAGIC,
            check_code: 0.into(),
            cmd,
            data,
        }
    }
}
