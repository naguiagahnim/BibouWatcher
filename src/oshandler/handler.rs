use std::time::Duration;

use device_query::{DeviceEvents, DeviceEventsHandler};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub struct BibouHandler {
    inactivitytimer: usize,
    inactivitycount: usize,
    device_state: Option<DeviceEventsHandler>,
}

impl Default for BibouHandler {
    fn default() -> Self {
        BibouHandler { 
            inactivitytimer: 0,
            inactivitycount: 0,
            device_state: None,
        }
    }
}

impl BibouHandler {
    pub async fn watch_for_events(&mut self, canceltoken: CancellationToken) {
        if self.device_state.is_none() {
            self.device_state = Some(DeviceEventsHandler::new(Duration::from_millis(10))
                .expect(""));

            let current_device_state = self.device_state.as_ref().unwrap();
            let _guard = current_device_state.on_mouse_move(|position| {
                println!("Mouse position: {:#?}", position);
            });

            loop {
                if canceltoken.is_cancelled() {
                    println!("Arrêt du Bibou Watcher");
                    break;
                }
                    
                sleep(Duration::from_secs(1)).await;
            }

            self.device_state = None;
        }
    }
}