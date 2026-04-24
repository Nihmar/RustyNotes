use notify::{Config, Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

pub struct FsWatcher {
    watcher: Option<notify::RecommendedWatcher>,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl FsWatcher {
    pub fn new() -> Self {
        FsWatcher {
            watcher: None,
            stop_tx: None,
        }
    }

    pub fn start(&mut self, path: &str, app_handle: AppHandle) -> Result<(), String> {
        let (stop_tx, stop_rx) = mpsc::channel();

        let watch_path = PathBuf::from(path);
        let app_clone = app_handle.clone();

        let (event_tx, event_rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = event_tx.send(event);
            }
        })
        .map_err(|e| e.to_string())?;

        watcher
            .configure(Config::default().with_poll_interval(Duration::from_secs(1)))
            .map_err(|e| e.to_string())?;

        watcher
            .watch(&watch_path, RecursiveMode::Recursive)
            .map_err(|e| e.to_string())?;

        self.watcher = Some(watcher);
        self.stop_tx = Some(stop_tx);

        std::thread::spawn(move || {
            let mut last_event: Option<(String, Instant)> = None;
            let debounce_ms = Duration::from_millis(100);

            loop {
                // Check stop signal
                if stop_rx.try_recv().is_ok() {
                    break;
                }

                match event_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(event) => {
                        let path_str = event.paths.first()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_default();

                        // Only care about .md files
                        if !path_str.ends_with(".md") {
                            continue;
                        }

                        // Debounce
                        let now = Instant::now();
                        if let Some((last_path, last_time)) = &last_event {
                            if last_path == &path_str && now.duration_since(*last_time) < debounce_ms {
                                continue;
                            }
                        }
                        last_event = Some((path_str.clone(), now));

                        let event_name = match event.kind {
                            EventKind::Create(_) => "note-created",
                            EventKind::Modify(_) => "note-modified",
                            EventKind::Remove(_) => "note-deleted",
                            _ => continue,
                        };

                        let payload = serde_json::json!({
                            "path": path_str,
                            "event": event_name
                        });

                        let _ = app_clone.emit(event_name, payload);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        self.stop_tx = None;
        self.watcher = None;
    }
}
