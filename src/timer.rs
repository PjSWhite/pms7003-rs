#[cfg(all(feature = "embedded-hal-impl", not(feature = "rp2040-impl")))]
use embedded_hal::delay::DelayNs;
#[cfg(all(feature = "rp2040-impl", not(feature = "embedded-hal-impl")))]
use rp2040_hal::fugit::MicrosDurationU32;

pub trait TimerAlarm {
    type Countdown;
    type Result;

    fn schedule(&mut self, countdown: Self::Countdown) -> Self::Result;
    fn from_seconds(secs: u32) -> Self::Countdown;
    fn is_ready(&self) -> bool;
}

pub struct NoAlarm;

impl TimerAlarm for NoAlarm {
    type Countdown = ();
    type Result = ();

    fn is_ready(&self) -> bool {
        true
    }

    fn from_seconds(_secs: u32) -> Self::Countdown {}

    fn schedule(&mut self, _countdown: Self::Countdown) -> Self::Result {}
}

#[cfg(all(feature = "rp2040-impl", not(feature = "embedded-hal-impl")))]
impl<T> TimerAlarm for T
where
    T: rp2040_hal::timer::Alarm,
{
    type Countdown = MicrosDurationU32;
    type Result = Result<(), rp2040_hal::timer::ScheduleAlarmError>;

    fn is_ready(&self) -> bool {
        self.finished()
    }

    fn from_seconds(secs: u32) -> Self::Countdown {
        MicrosDurationU32::secs(secs)
    }

    fn schedule(&mut self, countdown: Self::Countdown) -> Self::Result {
        self.schedule(countdown)
    }
}

#[cfg(all(feature = "embedded-hal-impl", not(feature = "rp2040-impl")))]
impl<T> TimerAlarm for T
where
    T: embedded_hal::delay::DelayNs,
{
    type Countdown = u32;
    type Result = ();

    fn is_ready(&self) -> bool {
        true
    }

    fn from_seconds(secs: u32) -> Self::Countdown {
        secs.saturating_mul(1000)
    }

    fn schedule(&mut self, countdown: Self::Countdown) -> Self::Result {
        self.delay_ms(countdown);
    }
}
