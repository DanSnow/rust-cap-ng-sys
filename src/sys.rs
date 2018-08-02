#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use libc;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Cap {
    Chown = 0,
    DacOverride = 1,
    DacReadSearch = 2,
    Fowner = 3,
    Fsetid = 4,
    Kill = 5,
    Setgid = 6,
    Setuid = 7,
    Setpcap = 8,
    LinuxImmutable = 9,
    NetBindService = 10,
    NetBroadcast = 11,
    NetAdmin = 12,
    NetRaw = 13,
    IpcLock = 14,
    IpcOwner = 15,
    SysModule = 16,
    SysRawio = 17,
    SysChroot = 18,
    SysPtrace = 19,
    SysPacct = 20,
    SysAdmin = 21,
    SysBoot = 22,
    SysNice = 23,
    SysResource = 24,
    SysTime = 25,
    SysTtyConfig = 26,
    Mknod = 27,
    Lease = 28,
    AuditWrite = 29,
    AuditControl = 30,
    Setfcap = 31,
    MacOverride = 32,
    MacAdmin = 33,
    Syslog = 34,
    WakeAlarm = 35,
    BlockSuspend = 36,
    AuditRead = 37,
}

impl Into<u32> for Cap {
    fn into(self: Self) -> u32 {
        self as u32
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum capng_act_t {
    CAPNG_DROP = 0,
    CAPNG_ADD = 1,
}

bitflags! {
    pub struct capng_type_t: u32 {
        const CAPNG_EFFECTIVE = 1;
        const CAPNG_PERMITTED = 2;
        const CAPNG_INHERITABLE = 4;
        const CAPNG_BOUNDING_SET = 8;
    }
}

bitflags! {
    pub struct capng_select_t: u32 {
        const CAPNG_SELECT_CAPS = 16;
        const CAPNG_SELECT_BOUNDS = 32;
        const CAPNG_SELECT_BOTH = Self::CAPNG_SELECT_CAPS.bits | Self::CAPNG_SELECT_BOUNDS.bits;
    }
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum capng_results_t {
    CAPNG_FAIL = -1,
    CAPNG_NONE = 0,
    CAPNG_PARTIAL = 1,
    CAPNG_FULL = 2,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum capng_print_t {
    CAPNG_PRINT_STDOUT = 0,
    CAPNG_PRINT_BUFFER = 1,
}

bitflags! {
    pub struct capng_flags_t: u32 {
        const CAPNG_NO_FLAG = 0;
        const CAPNG_DROP_SUPP_GRP = 1;
        const CAPNG_CLEAR_BOUNDING = 2;
        const CAPNG_INIT_SUPP_GRP = 4;
    }
}

extern "C" {
    pub(crate) fn capng_clear(set: u32);
    pub(crate) fn capng_fill(set: u32);
    pub fn capng_setpid(pid: libc::c_int);
    pub fn capng_get_caps_process() -> libc::c_int;
    pub(crate) fn capng_update(
        action: capng_act_t,
        type_: u32,
        capability: libc::c_uint,
    ) -> libc::c_int;
    pub(crate) fn capng_apply(set: u32) -> libc::c_int;
    pub fn capng_lock() -> libc::c_int;
    pub(crate) fn capng_change_id(uid: libc::c_int, gid: libc::c_int, flag: u32) -> libc::c_int;
    pub fn capng_get_caps_fd(fd: libc::c_int) -> libc::c_int;
    pub fn capng_apply_caps_fd(fd: libc::c_int) -> libc::c_int;
    pub(crate) fn capng_have_capabilities(set: u32) -> capng_results_t;
    pub(crate) fn capng_have_capability(which: u32, capability: libc::c_uint) -> libc::c_int;
    pub(crate) fn capng_print_caps_numeric(where_: capng_print_t, set: u32) -> *mut libc::c_char;
    pub(crate) fn capng_print_caps_text(where_: capng_print_t, which: u32) -> *mut libc::c_char;
    pub fn capng_name_to_capability(name: *const libc::c_char) -> libc::c_int;
    pub(crate) fn capng_capability_to_name(capability: libc::c_uint) -> *const libc::c_char;
    pub fn capng_save_state() -> *mut libc::c_void;
    pub fn capng_restore_state(state: *mut *mut libc::c_void);
}
