use std::fs;
use tracing_subscriber::fmt;
use tracing_appender::non_blocking::WorkerGuard;

static mut LOG_GUARD: Option<WorkerGuard> = None;

pub fn init_logger() {
    if !cfg!(debug_assertions) {
        return;
    }

    let _ = fs::create_dir_all("logs");

    let file_appender = tracing_appender::rolling::never("logs", "dev.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    unsafe {
        LOG_GUARD = Some(guard);
    }

    fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false)
        .with_target(false)
        .init();
}
