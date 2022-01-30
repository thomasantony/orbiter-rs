/// File I/O functions for reading/writing to scenario and configuration files
use crate::ffi;
use crate::vector::Vector3;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct FileHandle(usize);
pub use ffi::FileAccessMode;
pub use ffi::PathRoot;

impl FileHandle {
    /// Open a file for reading or writing
    pub fn open(filename: &str, mode: FileAccessMode, root: PathRoot) -> Self {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::oapiOpenFile(filename.as_ptr(), mode, root) }
    }
    /// Read the value of a tag from a configuration file as a [String]
    pub fn read_string(&self, item_name: &str) -> String {
        let mut buffer = vec![0; 256];
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiReadItem_string(
                self.clone(),
                item_name.as_ptr() as *mut c_char,
                buffer.as_mut_ptr(),
            )
        };
        unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .to_string()
    }
    /// Read the value of a tag from a configuration file as a [f64]
    pub fn read_f64(&self, item_name: &str) -> f64 {
        let mut val: f64 = 0.0;
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiReadItem_float(self.clone(), item_name.as_ptr() as *mut c_char, &mut val)
        };
        val
    }
    /// Read the value of a tag from a configuration file as a [i32]
    pub fn read_i32(&self, item_name: &str) -> i32 {
        let mut val: i32 = 0;
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiReadItem_int(self.clone(), item_name.as_ptr() as *mut c_char, &mut val) };
        val
    }
    /// Read the value of a tag from a configuration file as a [bool]
    pub fn read_bool(&self, item_name: &str) -> bool {
        let mut val: bool = false;
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiReadItem_bool(self.clone(), item_name.as_ptr() as *mut c_char, &mut val)
        };
        val
    }
    /// Read the value of a tag from a configuration file as a [Vector3]
    pub fn read_vec(&self, item_name: &str) -> Vector3 {
        let mut val = Vector3::default();
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiReadItem_vec(self.clone(), item_name.as_ptr() as *mut c_char, &mut val) };
        val
    }
    /// Write a tag and its [String] value to a configuration file
    pub fn write_string(&self, item_name: &str, val: &str) {
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiWriteItem_string(
                self.clone(),
                item_name.as_ptr() as *mut c_char,
                val.as_ptr() as *mut c_char,
            )
        };
    }
    /// Write a tag and its [f64] value to a configuration file
    pub fn write_f64(&self, item_name: &str, val: f64) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_float(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Write a tag and its [i32] value to a configuration file
    pub fn write_i32(&self, item_name: &str, val: i32) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_int(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Write a tag and its [bool] value to a configuration file
    pub fn write_bool(&self, item_name: &str, val: bool) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_bool(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Write a tag and its [Vector3] value to a configuration file
    pub fn write_vec(&self, item_name: &str, val: &Vector3) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_vec(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Writes a line to a file
    pub fn write(&self, line: &[u8])
    {
        let line = CString::new(line).unwrap();
        unsafe { ffi::oapiWriteLine(self.clone(), line.as_ptr() as *mut c_char) };
    }

    /// Write a parameter and its [String] value to a configuration file
    pub fn write_scenario_string(&self, item_name: &str, val: &str) {
        let item_name = CString::new(item_name).unwrap();
        let val = CString::new(val).unwrap();
        unsafe {
            ffi::oapiWriteScenario_string(
                self.clone(),
                item_name.as_ptr() as *mut c_char,
                val.as_ptr() as *mut c_char,
            )
        };
    }
    /// Write a parameter and its [f64] value to a scenario file
    pub fn write_scenario_f64(&self, item_name: &str, val: f64) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteScenario_float(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Write a parameter and its [i32] value to a scenario file
    pub fn write_scenario_i32(&self, item_name: &str, val: i32) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteScenario_int(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    /// Write a parameter and its [Vector3] value to a scenario file
    pub fn write_scenario_vec(&self, item_name: &str, val: &Vector3) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteScenario_vec(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }

    /// Close a file after reading or writing
    pub fn close(self, mode: FileAccessMode) {
        ffi::oapiCloseFile(self, mode);
    }
}
