use slint_interpreter::{SharedString, Value};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::AppState;
use crate::services::clock::ClockService;
use super::Widget;

pub struct ClockWidget {
    pub service: Arc<Mutex<ClockService>>,
}

impl ClockWidget {
    pub fn new(service: Arc<Mutex<ClockService>>) -> Self {
        Self { service }
    }
}

impl Widget for ClockWidget {
    fn name(&self) -> &str {
        "clock"
    }

    fn update(&self, state: Rc<RefCell<AppState>>) {
        let now = if let Ok(svc) = self.service.lock() {
            svc.current_time.clone()
        } else {
            "--:--:--".to_string()
        };
        
        let st = state.borrow();
        for bar in &st.bar_instances {
            if let Some(ui) = bar.upgrade() {
                let _ = ui.set_property("time-text", Value::from(SharedString::from(now.clone())));
            }
        }
    }
}
