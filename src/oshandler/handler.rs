use std::time::Duration;

use device_query::{DeviceEvents, DeviceEventsHandler};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub struct BibouHandler {
    inactivitytimer: usize,
    inactivitycount: usize,
}

impl Default for BibouHandler {
    fn default() -> Self {
        BibouHandler { 
            inactivitytimer: 0,
            inactivitycount: 0,
        }
    }
}

impl BibouHandler {
        pub async fn watch_for_events(canceltoken: CancellationToken) {
        {
            let device_state = DeviceEventsHandler::new(Duration::from_millis(10))
                .expect("Failed to start event loop");

            let _guard = device_state.on_mouse_move(|position| {
                println!("Mouse position: {:#?}", position);
            });

            loop {
                if canceltoken.is_cancelled() {
                    println!("Arrêt du Bibou Watcher");
                    break;
                }
                    
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}