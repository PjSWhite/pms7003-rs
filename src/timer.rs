pub struct NoAlarm;

impl TimerAlarm for NoAlarm {
    type Countdown = ();
    type Result = ();

    fn is_ready(&self) -> bool {
        true
    }

    fn schedule(&mut self, _countdown: Self::Countdown) -> Self::Result {}
}

pub trait TimerAlarm {
    type Countdown;
    type Result;

    fn schedule(&mut self, countdown: Self::Countdown) -> Self::Result;
    fn is_ready(&self) -> bool;
}
