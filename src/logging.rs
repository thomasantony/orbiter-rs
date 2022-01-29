/// Logging handler that routes logs to the Orbiter log

use log::{Level, Log, LevelFilter, Metadata, Record};
use std::ffi::CString;
use std::os::raw::c_char;
use crate::ffi;

pub struct OrbiterLogger {    
}

impl Log for OrbiterLogger {
    /// Determines if a log message with the specified metadata would be
    /// logged.
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// discarded anyway.
    fn enabled(&self, metadata: &Metadata) -> bool
    {
        // metadata.level() <= self.log_level_filter() && self.includes_module(metadata.target())
        metadata.level() <= LevelFilter::Debug
    }

    /// Logs the `Record`.
    ///
    /// Note that `enabled` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    fn log(&self, record: &Record)
    {
        if self.enabled(record.metadata())
        {
            let message = format!("{}: {}", record.level().to_string(), record.args());
            let message_cstr = CString::new(message).unwrap();
            unsafe { ffi::oapiWriteLog(message_cstr.as_ptr() as *mut c_char) }
        }
    }

    /// Flushes any buffered records.
    fn flush(&self)
    {

    }
}
