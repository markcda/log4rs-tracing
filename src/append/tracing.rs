//! The tracing console appender.
//!
//! Requires the `tracing_console_appender` feature.

use log::Record;
use tracing_log::AsTrace;

use crate::append::Append;

/// An appender which logs to `tracing` library.
///
/// Any `log` event should be handled both by `log4rs` and `tracing` libraries.
#[derive(Debug)]
pub struct TracingAppender {
    ignore_crates: Vec<String>,
}

impl Append for TracingAppender {
    fn append(&self, record: &Record) -> anyhow::Result<()> {
        if self.enabled(record.metadata()) {
            tracing_log::dispatch_record(record);
        }
        Ok(())
    }

    fn flush(&self) {}
}

impl TracingAppender {
    /// Creates a new `ConsoleAppender` builder.
    pub fn builder() -> TracingAppenderBuilder {
        Default::default()
    }

    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        // Okay, it wasn't disabled by the max level — do we have any specific
        // modules to ignore?
        if !self.ignore_crates.is_empty() {
            // If we are ignoring certain module paths, ensure that the metadata
            // does not start with one of those paths.
            let target = metadata.target();
            for ignored in &self.ignore_crates[..] {
                if target.starts_with(ignored) {
                    return false;
                }
            }
        }

        // Finally, check if the current `tracing` dispatcher cares about this.
        tracing::dispatch::get_default(|dispatch| dispatch.enabled(&metadata.as_trace()))
    }
}

/// A builder for `TracingAppender`s.
#[derive(Default)]
pub struct TracingAppenderBuilder {
    ignore_crates: Vec<String>,
}

impl TracingAppenderBuilder {
    /// Sets the list of ignoring by `tracing` library crates.
    pub fn ignore_crates(mut self, ignore_crates: Vec<String>) ->TracingAppenderBuilder {
        self.ignore_crates = ignore_crates;
        self
    }

    /// Consumes the `TracingAppenderBuilder`, producing a `TracingAppender`.
    pub fn build(self) -> TracingAppender {
        TracingAppender {
            ignore_crates: self.ignore_crates,
        }
    }
}
