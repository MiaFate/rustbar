pub trait Service {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn update(&mut self) {}
}

pub mod clock;
pub mod notifications;
