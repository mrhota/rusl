use libc::*;
use pthread::__pthread_self;

#[inline(always)]
pub unsafe fn set_errno(e: c_int) { *__errno_location() = e; }

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut c_int { &mut ((*__pthread_self()).errno_val) }
