use libc::{c_int, c_void, c_long, EINTR};
use platform::{syscall_arg_t, PTHREAD_CANCEL_DISABLE};
use platform::pthread::__pthread_self;
use thread::__syscall_cp_asm;
use thread::pthread::pthread;
use thread::pthread_cancel::__cancel;
use core::intrinsics;

use syscall_mgt::syscall_return;
use syscall::syscall6;

#[allow(unused_variables)]
#[no_mangle]
pub unsafe extern "C" fn __syscall_cp(nr: syscall_arg_t,
                                      u: syscall_arg_t,
                                      v: syscall_arg_t,
                                      w: syscall_arg_t,
                                      x: syscall_arg_t,
                                      y: syscall_arg_t,
                                      z: syscall_arg_t) -> c_long {
    __syscall_cp_c(nr, u, v, w, x, y, z)
}

#[no_mangle]
pub unsafe extern "C" fn __syscall_cp_c(nr: syscall_arg_t,
                                        u: syscall_arg_t,
                                        v: syscall_arg_t,
                                        w: syscall_arg_t,
                                        x: syscall_arg_t,
                                        y: syscall_arg_t,
                                        z: syscall_arg_t) -> c_long {
    let this = __pthread_self();
    let st = (*this).canceldisable;
    let is_close = nr as usize == ::syscall::nr::CLOSE;

    if st != 0 && (st == PTHREAD_CANCEL_DISABLE || is_close) {
        return syscall_return(
            syscall6(nr as usize,
                     u as usize, v as usize, w as usize,
                     x as usize, y as usize, z as usize)) as c_long;
    }

    let p = &(*this).cancel as *const c_int as *const c_void;
    let mut r = __syscall_cp_asm(p, nr, u, v, w, x, y, z);
    if r == -(EINTR as c_long) && !is_close && (*this).cancel != 0 && (*this).canceldisable != PTHREAD_CANCEL_DISABLE {
        r = __cancel();
    }
    r
}
