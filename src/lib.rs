#[macro_use]
extern crate bitflags;
extern crate libc;

mod sys;
pub use self::sys::Cap;
use std::ffi::CStr;

pub use self::sys::{
    capng_act_t, capng_flags_t, capng_print_t, capng_results_t, capng_select_t, capng_type_t,
};

#[inline]
pub unsafe fn capng_clear(set: capng_select_t) {
    sys::capng_clear(set.bits());
}

#[inline]
pub unsafe fn capng_fill(set: capng_select_t) {
    sys::capng_fill(set.bits());
}

pub use self::sys::capng_setpid;

pub use self::sys::capng_get_caps_process;

#[inline]
pub unsafe fn capng_update(
    action: capng_act_t,
    type_: capng_type_t,
    capability: Cap,
) -> libc::c_int {
    self::sys::capng_update(action, type_.bits(), capability.into())
}

#[inline]
pub unsafe fn capng_apply(set: capng_select_t) -> libc::c_int {
    self::sys::capng_apply(set.bits())
}

pub use self::sys::capng_lock;

#[inline]
pub unsafe fn capng_change_id(
    uid: libc::c_int,
    gid: libc::c_int,
    flag: capng_flags_t,
) -> libc::c_int {
    self::sys::capng_change_id(uid, gid, flag.bits())
}

pub use self::sys::{capng_apply_caps_fd, capng_get_caps_fd};

#[inline]
pub unsafe fn capng_have_capabilities(set: capng_select_t) -> capng_results_t {
    self::sys::capng_have_capabilities(set.bits())
}

#[inline]
pub unsafe fn capng_have_capability(which: capng_type_t, capability: Cap) -> libc::c_int {
    self::sys::capng_have_capability(which.bits(), capability.into())
}

#[inline]
pub unsafe fn capng_print_caps_numeric(
    where_: capng_print_t,
    set: capng_select_t,
) -> *mut libc::c_char {
    self::sys::capng_print_caps_numeric(where_, set.bits())
}

#[inline]
pub unsafe fn capng_print_caps_text(
    where_: capng_print_t,
    which: capng_type_t,
) -> *mut libc::c_char {
    self::sys::capng_print_caps_text(where_, which.bits())
}

pub fn capng_capability_to_name(capability: Cap) -> Option<&'static str> {
    let name = unsafe { self::sys::capng_capability_to_name(capability.into()) };
    if name.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
}

pub use self::sys::{capng_name_to_capability, capng_restore_state, capng_save_state};
