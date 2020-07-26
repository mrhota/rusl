#![no_std]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(untagged_unions)]

#![allow(non_camel_case_types)]

extern crate compiler_builtins;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate rlibc;
extern crate spin;
extern crate sc as syscall;
extern crate va_list;

pub use rlibc::*;

#[macro_use]
pub mod syscall_mgt;

pub mod exit;
pub mod fcntl;
pub mod malloc;
pub mod mmap;
pub mod string;
pub mod thread;
pub mod time;
pub mod unistd;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

pub use platform::atomic;
pub use platform::errno;
pub use platform::environ;
pub use platform::mman;
pub use platform::pthread;
pub use platform::signal;

use core::intrinsics;
#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
