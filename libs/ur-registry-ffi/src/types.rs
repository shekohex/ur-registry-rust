use std::ffi::c_void;
use std::os::raw::{c_char, c_uint};

pub type ErrorCallback = extern "C" fn(error: *mut c_char);

pub type PtrVoid = *mut c_void;
pub type PtrString = *mut c_char;
pub type PtrU32 = *mut c_uint;