#![no_std]
#![warn(clippy::expect_used)]

pub mod driver;
mod error;
pub mod frame;
pub mod timer;

pub use driver::Pms7003Controller;
pub use error::Error;
pub use frame::Pms7003DataFrame;
pub use timer::{NoAlarm, TimerAlarm};

use zerocopy::{Ref, byteorder};

type PmsU16Int = byteorder::U16<byteorder::BigEndian>;
pub type DataFrameResult<'a> = Result<Ref<&'a [u8], Pms7003DataFrame>, Error>;
