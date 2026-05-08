use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;

static DEBUG_LOG: AtomicBool = AtomicBool::new(false);
static LOGGER: Mutex<Option<LoggerState>> = Mutex::new(None);

struct LoggerState {
    started_at: Instant,
    last_log_at: Instant,
}

pub fn set_debug_log(enabled: bool) {
    DEBUG_LOG.store(enabled, Ordering::Relaxed);

    if enabled {
        let now = Instant::now();
        *LOGGER.lock().unwrap() = Some(LoggerState {
            started_at: now,
            last_log_at: now,
        });
    }
}

pub fn debug_log_message(args: std::fmt::Arguments<'_>) {
    if !DEBUG_LOG.load(Ordering::Relaxed) {
        return;
    }

    let now = Instant::now();
    let mut logger = LOGGER.lock().unwrap();

    let Some(state) = logger.as_mut() else {
        *logger = Some(LoggerState {
            started_at: now,
            last_log_at: now,
        });
        eprintln!("[debug +0.000s | +0.000s] {}", args);
        return;
    };

    let since_start = now.duration_since(state.started_at);
    let since_last = now.duration_since(state.last_log_at);
    state.last_log_at = now;

    eprintln!(
        "[debug +{:.3}s | +{:.3}s] {}",
        since_start.as_secs_f64(),
        since_last.as_secs_f64(),
        args
    );
}

#[macro_export]
macro_rules! debug_log {
      ($($arg:tt)*) => {
          crate::logger::debug_log_message(format_args!($($arg)*))
      };
  }
