use ns_error::NsError;
use std::ffi::{c_double, c_float, c_int};
use std::io::{self};

// Helper function to read a line from stdin safely
fn read_line_buffer() -> Result<String, NsError> {
    let mut buffer = String::new();

    // If reading from stdin completely fails (e.g., broken pipe), return the specific error
    if io::stdin().read_line(&mut buffer).is_err() {
        return Err(NsError::IoReadFailed);
    }

    Ok(buffer.trim().to_string())
}

/// Read Integer
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_int`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_int(ptr: *mut c_int) -> NsError {
    if ptr.is_null() {
        return NsError::Any; // Prevents segfaults, flags an error
    }

    match read_line_buffer() {
        Ok(input) => {
            // Match the parse result instead of blindly unwrapping!
            match input.parse::<c_int>() {
                Ok(val) => {
                    unsafe {
                        *ptr = val;
                    }
                    NsError::Success
                }
                Err(_) => NsError::InvalidInput,
            }
        }
        Err(e) => e,
    }
}

/// Read float
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_float`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_float(ptr: *mut c_float) -> NsError {
    if ptr.is_null() {
        return NsError::Any;
    }

    match read_line_buffer() {
        Ok(input) => match input.parse::<c_float>() {
            Ok(val) => {
                unsafe {
                    *ptr = val;
                }
                NsError::Success
            }
            Err(_) => NsError::InvalidInput,
        },
        Err(e) => e,
    }
}

/// Read double
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_double`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_double(ptr: *mut c_double) -> NsError {
    if ptr.is_null() {
        return NsError::Any;
    }

    match read_line_buffer() {
        Ok(input) => match input.parse::<c_double>() {
            Ok(val) => {
                unsafe {
                    *ptr = val;
                }
                NsError::Success
            }
            Err(_) => NsError::InvalidInput,
        },
        Err(e) => e,
    }
}
