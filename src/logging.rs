use crate::ffi;
/// Logging handler that routes logs to the Orbiter log
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::ffi::CString;
use std::os::raw::c_char;

pub struct OrbiterLogger {
    verbosity: LevelFilter,
    quiet: bool,
}
impl OrbiterLogger {
    pub fn new() -> Self {
        Self {
            verbosity: LevelFilter::Error,
            quiet: false,
        }
    }
    /// Set minimum log level
    pub fn level(&mut self, level: Level) -> &mut Self {
        let log_lvl = match level {
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        };

        self.verbosity = log_lvl;
        self
    }
    /// silence all output, no matter the value of verbosity
    pub fn quiet(&mut self, quiet: bool) -> &mut Self {
        self.quiet = quiet;
        self
    }

    fn log_level_filter(&self) -> LevelFilter {
        if self.quiet {
            LevelFilter::Off
        } else {
            self.verbosity
        }
    }

}
impl Log for OrbiterLogger {
    /// Determines if a log message with the specified metadata would be
    /// logged
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// discarded anyway
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level_filter()
    }

    /// Logs the `Record` based on current log level
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{}: {}", record.level().to_string(), record.args());
            let message_cstr = CString::new(message).unwrap();
            unsafe { ffi::oapiWriteLog(message_cstr.as_ptr() as *mut c_char) }
        }
    }

    /// Flushes any buffered records. Nothing to do here
    fn flush(&self) {}
}

impl Default for OrbiterLogger {
    fn default() -> Self {
        OrbiterLogger::new()
    }
}
