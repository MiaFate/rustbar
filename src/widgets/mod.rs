use slint_interpreter::{ComponentInstance};
use crate::AppState;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Widget {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    
    // Inicialización del widget (ej. registrar callbacks en Slint)
    fn init(&self, _instance: &ComponentInstance, _state: Rc<RefCell<AppState>>) {}
    
    // Actualización periódica (ej. cada segundo)
    fn update(&self, _state: Rc<RefCell<AppState>>) {}
}

pub mod clock;
pub mod notifications;
