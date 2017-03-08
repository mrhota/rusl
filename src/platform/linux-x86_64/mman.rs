use libc::*;

pub const PAGE_SIZE: off_t = 4096;
pub const MCL_ONFAULT: c_int = 4;
pub const MADV_SOFT_OFFLINE: c_int = 101;
pub const MLOCK_ONFAULT: c_int = 0x01;
