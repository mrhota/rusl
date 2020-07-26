use libc::*;
use platform::*;
use pthread::__pthread_self;
use thread::pthread_create::pthread_exit;
use thread::pthread::pthread;
use signal::{__sa_handler, sigaction, sigset_t};

extern "C" fn dummy() {}

#[no_mangle]
pub unsafe extern "C" fn __cancel() -> c_long {
    let this = __pthread_self();
    if (*this).canceldisable == PTHREAD_CANCEL_ENABLE || (*this).cancelasync != 0 {
        pthread_exit(PTHREAD_CANCELED);
    }
    (*this).canceldisable = PTHREAD_CANCEL_DISABLE;
    return -ECANCELED as c_long;
}

#[no_mangle]
pub unsafe extern "C" fn __testcancel() {
    let this = __pthread_self();
    if (*this).cancel != 0 && (*this).canceldisable == 0 {
        __cancel();
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_cancel(t: *const pthread) -> c_int {
    static mut INIT: bool = false;
    if !INIT {
        init_cancellation();
        INIT = true;
    }
    unimplemented!();
}

unsafe fn init_cancellation() {
    let masks_all_set = sigset_t::all_set();
    let sa = sigaction {
        __sa_handler: __sa_handler { sa_sigaction: cancel_handler },
        sa_flags: SA_SIGINFO | SA_RESTART,
        sa_mask: masks_all_set,
        sa_restorer: dummy,
    };
}

unsafe extern "C" fn cancel_handler(sig: c_int, si: *const siginfo_t, ctx: *const c_void) {
    unimplemented!();
}
