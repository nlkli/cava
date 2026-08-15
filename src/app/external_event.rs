use notify::{RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use winit::event_loop::EventLoopProxy;

use crate::config::Config;

#[derive(Debug)]
pub struct EventProducerHandle {
    stop_flag: Arc<AtomicBool>,
    jh: Option<std::thread::JoinHandle<()>>,
}

impl EventProducerHandle {
    pub fn stop(&mut self) -> Option<std::thread::JoinHandle<()>> {
        self.stop_flag.store(true, Ordering::Relaxed);
        self.jh.take()
    }

    pub fn stop_and_join(&mut self) -> std::thread::Result<()> {
        match self.stop() {
            Some(jh) => jh.join(),
            None => Ok(()),
        }
    }
}

impl Drop for EventProducerHandle {
    fn drop(&mut self) {
        let _ = self.stop_and_join();
    }
}

pub fn spawn_external_event_producer(
    proxy: EventLoopProxy<ExternalEvent>,
    config_path: Option<PathBuf>,
) -> EventProducerHandle {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_thread = stop_flag.clone();

    let watcher_proxy = proxy.clone();
    let mut last_event = Instant::now() - Duration::from_secs(10);

    let watcher_result = {
        let config_path = config_path
            .clone()
            .and_then(|p| std::fs::canonicalize(&p).ok());
        notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            let event = match res {
                Ok(event) => event,
                Err(_) => return,
            };

            if !matches!(event.kind, notify::EventKind::Modify(_)) {
                return;
            }

            if last_event.elapsed() <= Duration::from_millis(100) {
                return;
            }
            last_event = Instant::now();

            for p in event.paths {
                let is_config_path = config_path.as_deref().is_some_and(|cfg| cfg == p);

                if is_config_path {
                    if let Ok(config) = Config::from_ini_file(&p, None) {
                        let _ = watcher_proxy.send_event(ExternalEvent::ChangeConfig(config));
                    }
                }
            }
        })
    };
    let jh = std::thread::spawn(move || {
        // Keep the watcher alive for the duration of the thread; it is dropped
        // (and stops watching) automatically when the thread exits.
        let _watcher = match (watcher_result, &config_path) {
            (Ok(mut watcher), Some(path)) => {
                if let Err(e) = watcher.watch(path, RecursiveMode::NonRecursive) {
                    eprintln!("failed to watch config file {path:?}: {e}");
                }
                Some(watcher)
            }
            (Ok(_), None) => None,
            (Err(e), _) => {
                eprintln!("failed to create file watcher: {e}");
                None
            }
        };

        while !stop_flag_thread.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(100));
            if proxy.send_event(ExternalEvent::None).is_err() {
                // Event loop is gone; no point continuing.
                break;
            }
        }
    });

    EventProducerHandle {
        stop_flag,
        jh: Some(jh),
    }
}

#[derive(Debug, Clone, Default)]
pub enum ExternalEvent {
    #[default]
    None,
    ChangeConfig(Config),
}
