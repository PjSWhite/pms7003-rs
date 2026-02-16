#![no_std]
#![warn(clippy::expect_used)]

pub mod driver;
pub mod frame;
pub mod timer;

pub use driver::Pms7003Controller;
pub use frame::Pms7003DataFrame;
pub use timer::{NoAlarm, TimerAlarm};

use zerocopy::byteorder;

type PmsU16Int = byteorder::U16<byteorder::BigEndian>;
