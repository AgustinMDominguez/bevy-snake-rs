use std::time::Duration;
use::bevy::prelude::{Timer, TimerMode, Resource};

#[derive(Resource)]
pub struct StepTimers {
    pub boost_timer: Timer,
    pub tick_timer: Timer
}

impl StepTimers {
    pub fn new() -> Self {
        StepTimers{
            boost_timer: Timer::from_seconds(
                0.08,
                TimerMode::Repeating
            ),
            tick_timer: Timer::from_seconds(
                Self::default_tick_time(),
                TimerMode::Repeating
            )
        }
    }

    pub fn increase_tick_speed(&mut self) {
        let min_duration = Duration::from_secs_f32(0.1);
        let new_duration = self.tick_timer.duration() - Duration::from_secs_f32(0.06);
        if new_duration > min_duration {
            self.tick_timer.set_duration(new_duration)
        }
    }

    pub fn reset_tick_speed(&mut self) {
        self.tick_timer.set_duration(Duration::from_secs_f32(Self::default_tick_time()));
    }

    fn default_tick_time() -> f32 { 0.5 }
}
