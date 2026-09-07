//! File-based logging setup shared by all binaries.
//!
//! [`init`] wires up a `tracing` subscriber with three sinks:
//! - stderr (preserving the previous `eprintln!`/console visibility),
//! - a daily-rolling error/diagnostic log under [`LOG_DIR`] (`mvp.log.YYYY-MM-DD`),
//! - a daily-rolling **timing** log (`timing.log.YYYY-MM-DD`) that receives only the
//!   per-command / per-event timing records emitted on the [`TIMING_TARGET`] target.
//!
//! Timing records are kept out of stderr and out of the general log so high-volume
//! batch jobs don't drown the console or the error log; use [`record_timing`] to
//! emit them.
//!
//! Binaries run with `apps/mvp/` as their working directory, so the relative
//! `logs/` path resolves to `apps/mvp/logs/` (git-ignored).
//!
//! Verbosity is controlled by the `BODUL_LOG` env var (falling back to `RUST_LOG`,
//! then [`DEFAULT_FILTER`]), consistent with the `dotenvy`-driven config elsewhere.
//!
//! Each `main` must bind the returned guard for the lifetime of the process, e.g.
//! `let _guard = mvp::logging::init();` — dropping it flushes the non-blocking file
//! writers.

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

/// Directory (relative to the working dir, i.e. `apps/mvp/`) where log files land.
pub const LOG_DIR: &str = "logs";

/// Rolling-file name prefix for the general log (`mvp.log.YYYY-MM-DD`).
pub const LOG_FILE_PREFIX: &str = "mvp.log";

/// Rolling-file name prefix for the timing log (`timing.log.YYYY-MM-DD`).
pub const TIMING_FILE_PREFIX: &str = "timing.log";

/// Tracing target used for timing records; routed exclusively to the timing log.
pub const TIMING_TARGET: &str = "timing";

/// Default log filter when neither `BODUL_LOG` nor `RUST_LOG` is set.
pub const DEFAULT_FILTER: &str = "info";

/// Guards for the non-blocking file writers; keep alive for the whole run.
pub struct LoggingGuards {
    _general: WorkerGuard,
    _timing: WorkerGuard,
}

/// True for events emitted on the timing target (routed to the timing log only).
fn is_timing(meta: &tracing::Metadata<'_>) -> bool {
    meta.target() == TIMING_TARGET
}

/// Initializes the global tracing subscriber (stderr + general log + timing log).
///
/// Returns the [`LoggingGuards`] for the non-blocking file writers; keep them alive
/// for the whole run (`let _guard = ...`). Tolerates being called more than once
/// (only the first call installs the subscriber); the returned guards are always
/// valid.
#[must_use]
pub fn init() -> LoggingGuards {
    // Best-effort: if the dir can't be created the file appenders surface the error
    // themselves; stderr logging still works.
    let _ = std::fs::create_dir_all(LOG_DIR);

    let general_appender = tracing_appender::rolling::daily(LOG_DIR, LOG_FILE_PREFIX);
    let (general_writer, general_guard) = tracing_appender::non_blocking(general_appender);

    let timing_appender = tracing_appender::rolling::daily(LOG_DIR, TIMING_FILE_PREFIX);
    let (timing_writer, timing_guard) = tracing_appender::non_blocking(timing_appender);

    let env_filter = EnvFilter::try_from_env("BODUL_LOG")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));

    // stderr and the general log get everything except timing records.
    let general_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(general_writer)
        .with_filter(filter_fn(|meta| !is_timing(meta)));
    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(filter_fn(|meta| !is_timing(meta)));
    // The timing log gets only timing records.
    let timing_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(timing_writer)
        .with_filter(filter_fn(is_timing));

    // `try_init` avoids panicking if a subscriber is already installed.
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(general_layer)
        .with(stderr_layer)
        .with(timing_layer)
        .try_init();

    LoggingGuards {
        _general: general_guard,
        _timing: timing_guard,
    }
}

/// Emits one timing record to the timing log: how long a command/event took to be
/// processed, tagged with its id, retailer code (`-` when unavailable), and status.
///
/// `kind` is `"command"` or `"event"`; `name` is the command/event type.
pub fn record_timing(kind: &str, name: &str, id: &str, retailer: Option<&str>, elapsed_ms: u64, status: &str) {
    tracing::info!(
        target: TIMING_TARGET,
        kind,
        name,
        id,
        elapsed_ms,
        status,
        retailer = retailer.unwrap_or("-"),
    );
}

/// Emits one timing record for a remote fetch: how long the network fetch of `url`
/// took, tagged with its retailer code (`-` when the fetch has no retailer context)
/// and status.
pub fn record_fetch(url: &str, retailer: Option<&str>, elapsed_ms: u64, status: &str) {
    tracing::info!(
        target: TIMING_TARGET,
        kind = "fetch",
        url,
        elapsed_ms,
        status,
        retailer = retailer.unwrap_or("-"),
    );
}
