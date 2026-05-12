use std::sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}};
use zbus::{connection, interface};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::widgets::notifications::NotificationData;
use super::Service;

pub struct NotificationService {
    pub history: Arc<Mutex<Vec<NotificationData>>>,
    pub next_id: Arc<AtomicU32>,
    rx: Receiver<(String, String)>,
}

impl NotificationService {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let history = Arc::new(Mutex::new(Vec::new()));
        let next_id = Arc::new(AtomicU32::new(1));
        
        let tx_dbus = tx.clone();
        let id_dbus = next_id.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let manager = NotificationManager {
                    sender: tx_dbus,
                    counter: id_dbus,
                };
                if let Ok(builder) = connection::Builder::session() {
                    if let Ok(_conn) = builder.name("org.freedesktop.Notifications")
                        .unwrap()
                        .serve_at("/org/freedesktop/Notifications", manager)
                        .unwrap()
                        .build()
                        .await 
                    {
                        loop { thread::sleep(Duration::from_secs(10)); }
                    }
                }
            });
        });

        Self { history, next_id, rx }
    }

    pub fn clear_all(&self) {
        if let Ok(mut h) = self.history.lock() {
            h.clear();
        }
    }

    pub fn dismiss(&self, id: i32) {
        if let Ok(mut h) = self.history.lock() {
            h.retain(|n| n.id != id);
        }
    }
}

impl Service for NotificationService {
    fn name(&self) -> &str {
        "notifications"
    }

    fn update(&mut self) {
        while let Ok((title, body)) = self.rx.try_recv() {
            let now = chrono::Local::now().format("%H:%M").to_string();
            let id = self.next_id.fetch_add(1, Ordering::SeqCst) as i32;
            let new_notif = NotificationData { id, title, body, time: now };
            
            if let Ok(mut h) = self.history.lock() {
                h.insert(0, new_notif);
            }
        }
    }
}

struct NotificationManager {
    sender: Sender<(String, String)>,
    counter: Arc<AtomicU32>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationManager {
    #[zbus(name = "Notify")]
    async fn notify(
        &self,
        _app_name: String,
        _replaces_id: u32,
        _app_icon: String,
        summary: String,
        body: String,
        _actions: Vec<String>,
        _hints: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        _expire_timeout: i32,
    ) -> u32 {
        let _ = self.sender.send((summary, body));
        self.counter.load(Ordering::SeqCst)
    }

    #[zbus(name = "GetCapabilities")]
    async fn get_capabilities(&self) -> Vec<String> {
        vec!["actions".to_string(), "body".to_string(), "body-markup".to_string()]
    }

    #[zbus(name = "GetServerInformation")]
    async fn get_server_information(&self) -> (String, String, String, String) {
        ("mako".to_string(), "noctalia-dev".to_string(), "0.1.0".to_string(), "1.2".to_string())
    }
}
