pub struct GameTimer {
    current_time: u64,
    last_time: u64,
    pub dt: f32,
}

impl GameTimer {
    pub fn new(current_time: u64) -> GameTimer {
        GameTimer {
            current_time: current_time,
            last_time: 0,
            dt: 0.0,
        }
    }

    pub fn update(&mut self, current_time: u64, frequency: u64) -> () {
        self.last_time = self.current_time;
        self.current_time = current_time;
        self.dt = ((self.current_time - self.last_time) * 1000 / frequency) as f32;
    }
}
