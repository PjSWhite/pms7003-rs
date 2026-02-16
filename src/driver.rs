use embedded_io::{Error, Read, ReadReady, Write};
use zerocopy::{FromBytes, IntoBytes, Ref};

use crate::frame::Pms7003CommandFrame;
use crate::{Pms7003DataFrame, PmsU16Int};

// type ActivatedUartDevice<D, P> = hal::uart::UartPeripheral<hal::uart::Enabled, D, P>;
pub(super) type ConversionError<'a> = zerocopy::ConvertError<
    zerocopy::AlignmentError<&'a [u8], Pms7003DataFrame>,
    zerocopy::SizeError<&'a [u8], Pms7003DataFrame>,
    core::convert::Infallible,
>;

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

    fn write_checksum(buf: &mut [u8]) {
        let rest_of_buf_len = buf.len() - 2;
        let checksum = Self::compute_checksum(buf);
        buf[..=rest_of_buf_len].copy_from_slice(&checksum);
    }

    pub fn data(&self) -> Result<Ref<&[u8], Pms7003DataFrame>, crate::Error> {
        Ref::<&[u8], Pms7003DataFrame>::from_bytes(&self.data_buffer)
            .map_err(|_| crate::Error::Conversion)
    }
}

impl<S, T> Pms7003Controller<S, T>
where
    S: Read + Write + ReadReady,
    T: crate::timer::TimerAlarm,
{
    pub fn passive(&mut self) -> Result<(), crate::Error> {
        let cmd = Pms7003CommandFrame::new(0xe1, 0.into());
        self.cmd_buffer.copy_from_slice(cmd.as_bytes());

        self.send_cmd()
    }

    pub fn active(&mut self) -> Result<(), crate::Error> {
        let cmd = Pms7003CommandFrame::new(0xe1, 1.into());
        self.cmd_buffer.copy_from_slice(cmd.as_bytes());

        self.send_cmd()
    }

    pub fn sleep(&mut self) -> Result<(), crate::Error> {
        let cmd = Pms7003CommandFrame::new(0xe4, 0.into());
        self.cmd_buffer.copy_from_slice(cmd.as_bytes());

        self.send_cmd()
    }

    pub fn wake(&mut self) -> Result<(), crate::Error> {
        let cmd = Pms7003CommandFrame::new(0xe4, 1.into());
        self.cmd_buffer.copy_from_slice(cmd.as_bytes());

        self.send_cmd()
    }

    pub fn read_passive(&mut self) -> Result<Ref<&[u8], Pms7003DataFrame>, crate::Error> {
        let cmd = Pms7003CommandFrame::new(0xe2, 0.into());
        self.cmd_buffer.copy_from_slice(cmd.as_bytes());

        self.send_cmd()?;

        self.uart
            .read(&mut self.data_buffer)
            .map_err(|e| crate::Error::Write(e.kind()))?;
        self.data()
    }

    fn send_cmd(&mut self) -> Result<(), crate::Error> {
        Self::write_checksum(&mut self.cmd_buffer);
        if self.timer.is_ready() {
            self.uart
                .write_all(&self.cmd_buffer)
                .map_err(|e| crate::Error::Write(e.kind()))
        } else {
            Ok(())
        }
    }
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

// pub enum Pms7003Command {
//     PassiveRead,
//     ChangeMode(TransmissionMode),
//     SleepSet(SleepMode),
// }

// #[repr(u16)]
// #[derive(Clone, Copy, Debug)]
// pub enum SleepMode {
//     Sleep = 0x0000,
//     WakeUp = 0x0001,
// }

// #[repr(u16)]
// #[derive(Clone, Copy, Debug)]
// pub enum TransmissionMode {
//     Passive = 0x0000,
//     Active = 0x0001,
// }

// impl From<Pms7003Command> for Pms7003CommandFrame {
//     fn from(value: Pms7003Command) -> Self {
//         let (cmd, data) = match value {
//             Pms7003Command::PassiveRead => (0xe2, 0u16),
//             Pms7003Command::ChangeMode(mode) => (0xe1, mode as u16),
//             Pms7003Command::SleepSet(mode) => (0xe4, mode as u16),
//         };

//         Self {
//             magic: PMS7003MAGIC.into(),
//             cmd,
//             data: data.into(),
//             check_code: 0.into(),
//         }
//     }
// }
