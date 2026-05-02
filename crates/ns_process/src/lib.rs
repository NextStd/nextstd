use ns_error::NsError;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::process::{Child, Command};

pub struct NsProcess {
    inner: Child,
}

/// # Safety
///
/// * `command` must be a valid, null-terminated C string or null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_process_spawn(command: *const c_char) -> *mut NsProcess {
    if command.is_null() {
        return std::ptr::null_mut();
    }

    let cmd_str = unsafe { CStr::from_ptr(command) }.to_string_lossy();

    // Spawn using sh -c to allow for easy argument passing from C
    let result = Command::new("sh").arg("-c").arg(cmd_str.as_ref()).spawn();

    match result {
        Ok(child) => Box::into_raw(Box::new(NsProcess { inner: child })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
///
/// * `proc` must be a valid pointer to an `NsProcess` spawned by this library, or null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_process_is_running(proc: *mut NsProcess) -> bool {
    if proc.is_null() {
        return false;
    }

    let ns_proc = unsafe { &mut *proc };

    matches!(ns_proc.inner.try_wait(), Ok(None))
}

/// # Safety
///
/// * `proc` must be a valid pointer to an `NsProcess` spawned by this library, or null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_process_kill(proc: *mut NsProcess) -> NsError {
    if proc.is_null() {
        return NsError::Any;
    }

    let ns_proc = unsafe { &mut *proc };

    match ns_proc.inner.kill() {
        Ok(_) => NsError::Success,
        Err(_) => NsError::Any,
    }
}

/// # Safety
///
/// * `proc` must be a valid pointer to an `NsProcess` spawned by this library.
/// * This function must not be called more than once on the same pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_process_free(proc: *mut NsProcess) {
    if !proc.is_null() {
        unsafe {
            let mut ns_proc = Box::from_raw(proc);

            let _ = ns_proc.inner.wait();
        }
    }
}
