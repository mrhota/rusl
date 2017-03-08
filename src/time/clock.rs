use core::u64;

use libc::*;

use time::timespec;

#[no_mangle]
pub unsafe extern "C" fn __clock_gettime(clock: clockid_t, spec: &mut timespec) -> c_int {
    clock_gettime(clock, spec)
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clock: clockid_t, spec: &mut timespec) -> c_int {
    let mut r = syscall!(CLOCK_GETTIME, clock, spec as *mut timespec) as c_int;

    if r == -ENOSYS {
        if clock == CLOCK_REALTIME {
            syscall!(GETTIMEOFDAY, spec as *mut timespec, 0);
            spec.tv_nsec *= 1000;
            return 0;
        }
        r = -EINVAL;
    }

    r
}

#[no_mangle]
pub unsafe extern "C" fn clock() -> clock_t {
    let mut spec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    if clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut spec) != 0 {
        return -1 as clock_t;
    }

    if spec.tv_sec as u64 > u64::MAX / 1_000_000 ||
       (spec.tv_nsec / 1000) as u64 > (u64::MAX - 1_000_000) * spec.tv_sec as u64 {
        return -1 as clock_t;
    }

    (spec.tv_sec * 1000000 + spec.tv_nsec / 1000) as clock_t
}

#[no_mangle]
pub unsafe extern "C" fn clock_getcpuclockid(pid: pid_t, clock: *mut clockid_t) -> c_int {
    let mut spec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let id = ((-pid - 1) * 8) + 2;
    let r = syscall!(CLOCK_GETRES, id, &mut spec as *mut timespec);
    if r != 0 {
        -(r as c_int)
    } else {
        *clock = id;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn clock_getres(clock: clockid_t, spec: &mut timespec) -> c_int {
    syscall!(CLOCK_GETRES, clock, spec as *mut timespec) as i32
}

#[no_mangle]
pub unsafe extern "C" fn clock_nanosleep(clock: clockid_t,
                                         flags: c_int,
                                         req: &timespec,
                                         rem: &mut timespec)
                                         -> c_int {
    -(syscall!(CLOCK_NANOSLEEP,
               clock,
               flags,
               req as *const timespec,
               rem as *mut timespec) as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn clock_settime(clock: clockid_t, spec: &timespec) -> c_int {
    syscall!(CLOCK_SETTIME, clock, spec as *const timespec) as c_int
}
