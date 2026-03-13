use std::os::raw::c_char;
use thiserror::Error;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum NsError {
    #[error("Success")]
    Success = 0,

    // Catch-all (Similar to python's "except Exception")
    #[error("Unknown Error")]
    Any = 1,

    // ns_io errors
    #[error("I/O Error: Failed to read input")]
    IoReadFailed = 10,

    #[error("I/O Error: Failed to write input")]
    IoWriteFailed = 11,

    // Invalid input
    #[error("I/O Error: Invalid Input format")]
    InvalidInput = 12,

    //ns_string errors
    #[error("String Error: Memory Allocation failed")]
    StringAllocFailed = 20,

    #[error("String Error: Invalid UTF-8 sequence detected")]
    StringInvalidUtf8 = 21,

    #[error("Error: Index out of bounds")]
    IndexOutOfBounds = 22,
}

// Helper functions
// This is added to help C easily print exactly what went wrong

#[unsafe(no_mangle)]
pub extern "C" fn ns_error_message(err: NsError) -> *const c_char {
    let msg = match err {
        NsError::Success => c"Success",
        NsError::Any => c"Unknown Error",
        NsError::IoReadFailed => c"I/O Error: Failed to read input",
        NsError::IoWriteFailed => c"I/O Error: Failed to write input",
        NsError::InvalidInput => c"I/O Error: Invalid Input format",
        NsError::StringAllocFailed => c"String Error: Memory allocation failed",
        NsError::StringInvalidUtf8 => c"String Error: Invalid UTF-8 sequence detected",
        NsError::IndexOutOfBounds => c"Error: Index Out of Bounds",
    };

    msg.as_ptr()
}
