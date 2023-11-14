// Timer
pub struct Timer {
    start_time: std::time::Instant,
    last_frame_time: std::time::Instant,
    delta_time: std::time::Duration,
    frames: Vec<std::time::Duration>,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: std::time::Duration::new(0, 0),
            frames: Vec::new(),
        }
    }

    // Measures the time between two frames.
    pub fn step(&mut self) {
        let now = std::time::Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.frames.push(self.delta_time);

        while self.frames.len() > 60 {
            self.frames.remove(0);
        }
    }

    // Returns the time between the last two frames.
    pub fn get_delta(&self) -> std::time::Duration {
        self.delta_time
    }

    // Returns the average delta time over the last second.
    pub fn get_average_delta(&self) -> std::time::Duration {
        let total: std::time::Duration = self.frames.iter().sum();
        total / self.frames.len() as u32
    }

    // Returns the precise amount of time since some time in the past.
    pub fn get_time(&self) -> std::time::Duration {
        std::time::Instant::now() - self.start_time
    }

    // Returns the value of a timer with microsecond precision.
    pub fn get_micro_time(&self) -> u128 {
        self.get_time().as_micros()
    }

    // Returns the current frames per second.
    pub fn get_fps(&self) -> f32 {
        1.0 / self.get_average_delta().as_secs_f32()
    }

    // Pauses the current thread for the specified amount of time.
    pub fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }

    // Returns the elapsed time in milliseconds since the Timer started
    pub fn elapsed_ms(&self) -> u32 {
        self.get_time().as_millis() as u32
    }
}