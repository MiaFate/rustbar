use layer_shika::prelude::*;
use slint_interpreter::{SharedString, Value, Struct, ComponentInstance};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use slint::{VecModel, ModelRc};
use crate::AppState;
use crate::services::notifications::NotificationService;
use super::Widget;

#[derive(Clone)]
pub struct NotificationData {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub time: String,
}

impl NotificationData {
    pub fn to_slint_struct(&self) -> Struct {
        let mut s = Struct::default();
        let _ = s.set_field("id".to_string(), Value::from(self.id));
        let _ = s.set_field("title".to_string(), Value::from(SharedString::from(self.title.clone())));
        let _ = s.set_field("body".to_string(), Value::from(SharedString::from(self.body.clone())));
        let _ = s.set_field("time".to_string(), Value::from(SharedString::from(self.time.clone())));
        s
    }
}

pub struct NotificationWidget {
    pub service: Arc<Mutex<NotificationService>>,
}

impl NotificationWidget {
    pub fn new(service: Arc<Mutex<NotificationService>>) -> Self {
        Self { service }
    }
}

impl Widget for NotificationWidget {
    fn name(&self) -> &str {
        "notifications"
    }

    fn init(&self, instance: &ComponentInstance, _state: Rc<RefCell<AppState>>) {
        let s_clear = self.service.clone();
        let _ = instance.set_callback("clear_all", move |_| {
            if let Ok(svc) = s_clear.lock() {
                svc.clear_all();
            }
            Value::default()
        });

        let s_dismiss = self.service.clone();
        let _ = instance.set_callback("dismiss_notif", move |args: &[Value]| {
            if let Some(id) = args.get(0).and_then(|v| v.clone().try_into().ok()) {
                if let Ok(svc) = s_dismiss.lock() {
                    svc.dismiss(id);
                }
            }
            Value::default()
        });
    }

    fn update(&self, state: Rc<RefCell<AppState>>) {
        let svc = self.service.lock().unwrap();
        let history_data = svc.history.lock().unwrap();
        
        let history: Vec<Value> = history_data.iter()
            .map(|n| Value::from(n.to_slint_struct()))
            .collect();
        let model = ModelRc::new(VecModel::from(history));
        let has_notif = !history_data.is_empty();

        let st = state.borrow();
        for bar in &st.bar_instances {
            if let Some(ui) = bar.upgrade() {
                let _ = ui.set_property("has-notifications", Value::from(has_notif));
                let _ = ui.set_property("notifications", Value::from(model.clone()));
            }
        }
    }
}
