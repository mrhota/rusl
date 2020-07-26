pub mod pthread;
pub mod pthread_cancel;
pub mod pthread_create;
pub mod vmlock;

use libc::*;
use platform::{C_INT_MAX, syscall_arg_t};
use atomic::{a_dec, a_inc, a_spin};

pub const FUTEX_WAIT: c_int = 0;
pub const FUTEX_WAKE: c_int = 1;
pub const FUTEX_FD: c_int = 2;
pub const FUTEX_REQUEUE: c_int = 3;
pub const FUTEX_CMP_REQUEUE: c_int = 4;
pub const FUTEX_WAKE_OP: c_int = 5;
pub const FUTEX_LOCK_PI: c_int = 6;
pub const FUTEX_UNLOCK_PI: c_int = 7;
pub const FUTEX_TRYLOCK_PI: c_int = 8;
pub const FUTEX_WAIT_BITSET: c_int = 9;
pub const FUTEX_PRIVATE: c_int = 128;
pub const FUTEX_CLOCK_REALTIME: c_int = 256;

/// Defined in platform assembly (see platform/.../__syscall_cp_asm.rs)
extern "C" {
    pub fn __syscall_cp_asm(cancel: *const c_void, nr: syscall_arg_t,
                            u: syscall_arg_t, v: syscall_arg_t, w: syscall_arg_t,
                            x: syscall_arg_t, y: syscall_arg_t, z: syscall_arg_t) -> c_long;
    pub fn __cancel();
    pub fn __cp_begin();
    pub fn __cp_end();
    pub fn __cp_cancel();
}

#[no_mangle]
pub unsafe extern "C" fn __wake(address: *mut c_void, count: c_int, private: c_int) {
    let private = if private != 0 { 128 } else { private };

    let count = if count < 0 { C_INT_MAX } else { count };

    let res = syscall!(FUTEX, address, FUTEX_WAKE | private, count);

    if res == ENOSYS as usize {
        syscall!(FUTEX, address, FUTEX_WAKE, count);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wait(address: *mut c_int,
                                waiters: *mut c_int,
                                val: c_int,
                                private: c_int) {

    let private = if private != 0 { FUTEX_PRIVATE } else { private };

    for _ in 0..100 {
        if !waiters.is_null() || *waiters != 0 {
            if *address == val {
                a_spin();
            } else {
                return;
            }
        }
    }

    if !waiters.is_null() {
        a_inc(waiters);
    }

    while *address == val {
        let first = syscall!(FUTEX, address, FUTEX_WAIT | private, val, 0);

        if first as c_int == -ENOSYS {
            syscall!(FUTEX, address, FUTEX_WAIT, val, 0);
        }
    }

    if !waiters.is_null() {
        a_dec(waiters);
    }
}
