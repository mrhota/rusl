#![allow(non_snake_case)]

use libc::*;

pub const NSIG: c_int = 65;
const SIGSET_LEN: usize = 16;

pub union __sa_handler {
    pub sa_handler: unsafe extern "C" fn(signo: c_int),
    pub sa_sigaction: unsafe extern "C" fn(signo: c_int, info: *const siginfo_t, context: *const c_void)
}

pub struct sigaction {
    pub __sa_handler: __sa_handler,
    pub sa_flags: c_int,
    pub sa_mask: sigset_t,
    pub sa_restorer: extern "C" fn()
}

pub struct sigset_t {
    __bits: [c_ulong; SIGSET_LEN] // unsigned long __bits[128/size_of(long)]
}

impl sigset_t {
    pub fn all_set() -> sigset_t {
        sigset_t {
            __bits: [0xFF as c_ulong; SIGSET_LEN],
        }
    }
}
