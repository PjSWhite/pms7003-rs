// #[cfg(all(feature = "rp2040-impl", not(feature = "embedded-hal-impl")))]
#[cfg(feature = "rp2040-impl")]
use rp2040_hal::fugit::MicrosDurationU32;

pub trait TimerAlarm {
    type Countdown;

    fn schedule(&mut self, countdown: Self::Countdown);
    fn from_seconds(secs: u32) -> Self::Countdown;
    fn is_ready(&self) -> bool;
}

pub struct NoAlarm;

impl TimerAlarm for NoAlarm {
    type Countdown = ();

    fn is_ready(&self) -> bool {
        true
    }

    fn from_seconds(_secs: u32) -> Self::Countdown {}

    fn schedule(&mut self, _countdown: Self::Countdown) {}
}

// #[cfg(all(feature = "rp2040-impl", not(feature = "embedded-hal-impl")))]
#[cfg(feature = "rp2040-impl")]
impl<T> TimerAlarm for T
where
    T: rp2040_hal::timer::Alarm,
{
    type Countdown = MicrosDurationU32;

    fn is_ready(&self) -> bool {
        self.finished()
    }

    fn from_seconds(secs: u32) -> Self::Countdown {
        MicrosDurationU32::secs(secs)
    }

    fn schedule(&mut self, countdown: Self::Countdown) {
        if self.schedule(countdown).is_err() {
            // Edge cases are mutually exclusive;
            // self.cancel only fails when the alarm
            // wasn't armed in the first place, and
            // self.schedule only fails when the
            // alarm is already armed (which should
            // have been cleared by self.cancel)
            self.cancel().ok();
            self.schedule(countdown).ok();
        }
    }
}
