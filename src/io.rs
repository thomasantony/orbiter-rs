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
    pub fn open(filename: &str, mode: FileAccessMode, root: PathRoot) -> Self {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::oapiOpenFile(filename.as_ptr(), mode, root) }
    }
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
    pub fn read_f64(&self, item_name: &str) -> f64 {
        let mut val: f64 = 0.0;
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiReadItem_float(self.clone(), item_name.as_ptr() as *mut c_char, &mut val)
        };
        val
    }
    pub fn read_i32(&self, item_name: &str) -> i32 {
        let mut val: i32 = 0;
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiReadItem_int(self.clone(), item_name.as_ptr() as *mut c_char, &mut val) };
        val
    }
    pub fn read_bool(&self, item_name: &str) -> bool {
        let mut val: bool = false;
        let item_name = CString::new(item_name).unwrap();
        unsafe {
            ffi::oapiReadItem_bool(self.clone(), item_name.as_ptr() as *mut c_char, &mut val)
        };
        val
    }
    pub fn read_vec(&self, item_name: &str) -> Vector3 {
        let mut val = Vector3::default();
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiReadItem_vec(self.clone(), item_name.as_ptr() as *mut c_char, &mut val) };
        val
    }
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
    pub fn write_f64(&self, item_name: &str, val: f64) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_float(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    pub fn write_i32(&self, item_name: &str, val: i32) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_int(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    pub fn write_bool(&self, item_name: &str, val: bool) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_bool(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    pub fn write_vec(&self, item_name: &str, val: &Vector3) {
        let item_name = CString::new(item_name).unwrap();
        unsafe { ffi::oapiWriteItem_vec(self.clone(), item_name.as_ptr() as *mut c_char, val) };
    }
    pub fn close(self, mode: FileAccessMode) {
        ffi::oapiCloseFile(self, mode);
    }
}
