mod widgets;
mod services;

use layer_shika::prelude::*;
use layer_shika::calloop::TimeoutAction;
use slint_interpreter::{ComponentInstance, ComponentHandle};
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::widgets::Widget;
use crate::widgets::clock::ClockWidget;
use crate::widgets::notifications::{NotificationWidget, NotificationData};
use crate::services::Service;
use crate::services::clock::ClockService;
use crate::services::notifications::NotificationService;

pub struct AppState {
    pub notifications: Vec<NotificationData>,
    pub next_id: i32,
    pub bar_instances: Vec<slint_interpreter::Weak<ComponentInstance>>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let ui_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/rustbar.slint");
    
    let mut shell = Shell::from_file(ui_path)
        .surface("MainWindow")
        .height(440)
        .anchor(AnchorEdges::top_bar())
        .exclusive_zone(40)
        .namespace("rustbar")
        .build()?;

    let state = Rc::new(RefCell::new(AppState {
        notifications: Vec::new(),
        next_id: 1,
        bar_instances: Vec::new(),
    }));

    // 1. Inicializar Servicios
    let clock_svc = Arc::new(Mutex::new(ClockService::new()));
    let notif_svc = Arc::new(Mutex::new(NotificationService::new()));
    
    let mut services: Vec<Arc<Mutex<dyn Service + Send>>> = vec![
        clock_svc.clone(),
        notif_svc.clone(),
    ];

    // 2. Inicializar Widgets con sus respectivos servicios
    let widgets: Vec<Box<dyn Widget>> = vec![
        Box::new(ClockWidget::new(clock_svc)),
        Box::new(NotificationWidget::new(notif_svc)),
    ];

    // Configurar instancias de barra
    shell.with_all_surfaces(|_, instance| {
        state.borrow_mut().bar_instances.push(instance.as_weak());
        
        for widget in &widgets {
            widget.init(instance, state.clone());
        }
    });

    let event_handle = shell.event_loop_handle();

    // 3. Bucle de actualización (Servicios -> Widgets)
    let state_loop = state.clone();
    event_handle.add_timer(Duration::from_millis(500), move |_, _| {
        // Actualizar Servicios
        for svc in &mut services {
            if let Ok(mut s) = svc.lock() {
                s.update();
            }
        }
        
        // Actualizar Widgets
        for widget in &widgets {
            widget.update(state_loop.clone());
        }
        
        TimeoutAction::ToDuration(Duration::from_millis(500))
    })?;

    println!("INFO: RustBar con Arquitectura de Servicios iniciado");
    shell.run().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}
