use core;
use libc::{c_int, c_long, c_void};

pub mod atomic;
pub mod environ;
pub mod errno;
pub mod malloc;
pub mod mman;
pub mod pthread;
pub mod signal;

pub type syscall_arg_t = c_long;

pub const INT_BYTES: usize = 4;
pub const LONG_BYTES: usize = 8;
pub const C_INT_MAX: i32 = core::i32::MAX;

pub const PTHREAD_CANCEL_ENABLE: c_int = 0;
pub const PTHREAD_CANCEL_DISABLE: c_int = 1;
pub const PTHREAD_CANCEL_MASKED: c_int = 2;
pub const PTHREAD_CANCELED: *const c_void = -1isize as *const c_void;

mod __syscall_cp_asm;
