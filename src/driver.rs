use embedded_io::{Error, Read, ReadExactError, ReadReady, Write};
use log::debug;
use zerocopy::{IntoBytes, Ref};

use crate::frame::Pms7003CommandFrame;
use crate::{Pms7003DataFrame, PmsU16Int};

pub type ReadResult<'a> = Result<Ref<&'a [u8], Pms7003DataFrame>, crate::Error>;
// type ActivatedUartDevice<D, P> = hal::uart::UartPeripheral<hal::uart::Enabled, D, P>;
// pub(super) type ConversionError<'a> = zerocopy::ConvertError<
//     zerocopy::AlignmentError<&'a [u8], Pms7003DataFrame>,
//     zerocopy::SizeError<&'a [u8], Pms7003DataFrame>,
//     core::convert::Infallible,
// >;

pub struct Pms7003Controller<S, T> {
    uart: S,
    timer: T,
    data_buffer: [u8; 32],
    cmd_buffer: [u8; 7],
}

impl<S, T> Pms7003Controller<S, T> {
    pub fn new(uart: S, timer: T) -> Self {
        Self {
            uart,
            timer,
            data_buffer: [0; 32],
            cmd_buffer: [0; 7],
        }
    }

    fn compute_checksum(buf: &[u8]) -> [u8; 2] {
        let sum: u16 = buf[..buf.len() - 2].iter().map(|&b| b as u16).sum();

        PmsU16Int::new(sum).to_bytes()
    }

    fn write_checksum(buf: &mut [u8]) {
        let rest_of_buf_len = buf.len() - 2;
        let checksum = Self::compute_checksum(buf);
        buf[rest_of_buf_len..].copy_from_slice(&checksum);
    }

    pub fn has_data(&self) -> bool {
        let start_code = PmsU16Int::from_bytes(self.data_buffer[..2].try_into().unwrap());
        start_code == crate::frame::PMS7003MAGIC
    }

    pub fn verify_data_frame(&self) -> bool {
        let computed_checksum = u16::from_be_bytes(Self::compute_checksum(&self.data_buffer));
        let checksum_provided = u16::from_be_bytes(
            self.data_buffer[self.data_buffer.len() - 2..]
                .try_into()
                .unwrap(),
        );

        computed_checksum == checksum_provided
    }

    pub fn data(&self) -> ReadResult<'_> {
        Ref::<&[u8], Pms7003DataFrame>::from_bytes(&self.data_buffer)
            .map_err(|_| crate::Error::Conversion)
    }

    pub fn timer(&self) -> &T {
        &self.timer
    }

    pub fn timer_mut(&mut self) -> &mut T {
        &mut self.timer
    }
}

impl<S, T> Pms7003Controller<S, T>
where
    S: Write + Read + ReadReady,
{
    fn read_buffer(&mut self) -> Result<(), crate::Error> {
        self.uart
            .read_exact(&mut self.data_buffer)
            .map_err(|e| match e {
                ReadExactError::UnexpectedEof => crate::Error::Conversion,
                ReadExactError::Other(e) => crate::Error::ReadWrite(e.kind()),
            })?;

        let magic_pos = self
            .data_buffer
            .windows(2)
            .position(|w| w == [0x42, 0x4d])
            .ok_or(crate::Error::Conversion)?;

        if magic_pos != 0 {
            self.data_buffer.rotate_left(magic_pos);
            let tail = &mut self.data_buffer[32 - magic_pos..];
            self.uart.read_exact(tail).map_err(|e| match e {
                ReadExactError::UnexpectedEof => crate::Error::Conversion,
                ReadExactError::Other(e) => crate::Error::ReadWrite(e.kind()),
            })?;
        }

        Ok(())
    }
}

impl<S, T> Pms7003Controller<S, T>
where
    S: Read + Write + ReadReady,
    T: crate::TimerAlarm,
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
        self.read_buffer()?;

        debug!("Raw data frame read from sensor: {:x?}", self.data_buffer);
        self.data()
    }

    fn send_cmd(&mut self) -> Result<(), crate::Error> {
        Self::write_checksum(&mut self.cmd_buffer);
        self.timer.schedule(T::from_seconds(30));

        self.uart
            .write_all(&self.cmd_buffer)
            .map_err(|e| crate::Error::ReadWrite(e.kind()))
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
