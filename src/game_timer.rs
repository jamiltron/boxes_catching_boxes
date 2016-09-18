pub struct GameTimer {
    accumulator: f32,
    current_time: u64,
    last_time: u64,
    pub dt: f32,
}

impl GameTimer {
    pub fn new(current_time: u64, time_step: f32) -> Self {
        GameTimer {
            accumulator: 0.0,
            current_time: current_time,
            last_time: 0,
            dt: time_step,
        }
    }

    pub fn update(&mut self, current_time: u64, frequency: u64) {
        self.last_time = self.current_time;
        self.current_time = current_time;
        self.accumulator += (self.current_time - self.last_time) as f32 * 1000.0 / frequency as f32;
    }

    pub fn should_update_physics(&self) -> bool {
        self.accumulator >= self.dt
    }

    pub fn decrement(&mut self) -> () {
        self.accumulator -= self.dt
    }
}
