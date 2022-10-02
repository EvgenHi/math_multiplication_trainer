use std::time::Instant;

pub struct AverageTime {
    count: f64,
    overall_time: std::time::Duration,
    time: std::time::Instant,
}

impl AverageTime {
    pub fn get_overall_time(&self) -> f64 {
        if self.count == 0f64 {
            return 0f64;
        }
        (self.overall_time.as_secs_f64() / self.count).truncate(1u32)
    }
    pub fn count_again(&mut self) {
        self.count += 1f64;
        self.overall_time += self.time.elapsed();
        self.time = Instant::now();
    }
    pub fn new() -> AverageTime {
        AverageTime {
            count: 0f64,
            overall_time: std::time::Duration::new(0u64, 0u32),
            time: Instant::now(),
        }
    }
    pub fn reset_count(&mut self) {
        self.count = 0f64;
    }
}

trait Trunc {
    fn truncate(&self, limit: u32) -> f64;
}
impl Trunc for f64 {
    fn truncate(&self, limit: u32) -> f64 {
        let mult = 10i32.pow(limit) as f64;
        (self * mult).floor() / mult
    }
}
