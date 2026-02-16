use embedded_io::{Read, ReadReady, Write};

use crate::PmsU16Int;
use crate::frame::Pms7003CommandFrame;

// type ActivatedUartDevice<D, P> = hal::uart::UartPeripheral<hal::uart::Enabled, D, P>;

const PMS7003MAGIC: u16 = 0x424d;

pub struct Pms7003Controller<S, T> {
    uart: S,
    timer: T,
    data_buffer: [u8; 32],
    cmd_buffer: [u8; 7],
}

impl<S, T> Pms7003Controller<S, T> {
    fn compute_checksum(buf: &[u8]) -> [u8; 2] {
        let sum: u8 = buf[..buf.len() - 2].iter().sum();

        PmsU16Int::new(sum as u16).to_bytes()
    }
}

impl<S, T> Pms7003Controller<S, T>
where
    S: Read + Write + ReadReady,
    T: crate::timer::TimerAlarm,
{
}

// impl<D: hal::uart::UartDevice, P: hal::uart::ValidUartPinout<D>> Pms7003Controller<D, P> {
//     pub fn send_cmd(&mut self, cmd: Pms7003Command) {
//         let cmd_frame: Pms7003CommandFrame = cmd.into();
//         let cf_bytes = cmd_frame.as_bytes();
//     }

// fn compute_checksum(buf: &[u8]) -> [u8; 2] {
//     let sum: u8 = buf[..buf.len() - 2].iter().sum();

//     PmsU16Int::new(sum as u16).to_bytes()
// }
// }

pub enum Pms7003Command {
    PassiveRead,
    ChangeMode(TransmissionMode),
    SleepSet(SleepMode),
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum SleepMode {
    Sleep = 0x0000,
    WakeUp = 0x0001,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum TransmissionMode {
    Passive = 0x0000,
    Active = 0x0001,
}

impl From<Pms7003Command> for Pms7003CommandFrame {
    fn from(value: Pms7003Command) -> Self {
        let (cmd, data) = match value {
            Pms7003Command::PassiveRead => (0xe2, 0u16),
            Pms7003Command::ChangeMode(mode) => (0xe1, mode as u16),
            Pms7003Command::SleepSet(mode) => (0xe4, mode as u16),
        };

        Self {
            magic: PMS7003MAGIC.into(),
            cmd,
            data: data.into(),
            check_code: 0.into(),
        }
    }
}
