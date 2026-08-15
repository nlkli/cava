mod app;
mod external_event;

use external_event::{EventProducerHandle, ExternalEvent, spawn_external_event_producer};

pub use app::run;
