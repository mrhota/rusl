use libc::*;
use platform::*;
use pthread::__pthread_self;
use thread::pthread::pthread;

#[no_mangle]
pub unsafe extern "C" fn pthread_exit(result: *const c_void) -> ! {
    unimplemented!();
}
