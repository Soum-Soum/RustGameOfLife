use bevy::ecs::system::Resource;
use chrono::{DateTime, Local, TimeDelta};

#[derive(Resource)]
pub struct SimpleClock {
    last_update_time: DateTime<Local>,
    time_step: TimeDelta,
    is_paused: bool,
}

impl SimpleClock {
    pub fn new(time_step: TimeDelta) -> Self {
        Self {
            last_update_time: Local::now(),
            time_step,
            is_paused: false,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused
    }

    pub fn should_update(&mut self) -> bool {
        if self.is_paused {
            return false;
        }
        let now = Local::now();
        let delta = now - self.last_update_time;
        let should_update = delta > self.time_step;
        if should_update {
            self.last_update_time = now;
        }
        return should_update;
    }
}
