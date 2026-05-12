use chrono::Local;
use super::Service;

pub struct ClockService {
    pub current_time: String,
}

impl ClockService {
    pub fn new() -> Self {
        Self {
            current_time: "--:--:--".to_string(),
        }
    }
}

impl Service for ClockService {
    fn name(&self) -> &str {
        "clock"
    }

    fn update(&mut self) {
        self.current_time = Local::now().format("%H:%M:%S").to_string();
    }
}
