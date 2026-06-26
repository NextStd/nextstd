use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int};

use ns_error::NsError;
use ns_string::{NsString, NsStringData, NsStringHeap, ns_string_free};
use reqwest::blocking::Client;

const BROWSER_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0";

#[repr(C)]
pub struct NsHttpResponse {
    pub body: NsString,
    pub status_code: c_int,
}

fn rust_str_to_ns(s: String) -> NsString {
    let bytes = s.into_bytes();
    let len = bytes.len();

    if len < 24 {
        let mut inline = [0; 24];
        inline[..len].copy_from_slice(&bytes);
        NsString {
            len,
            is_heap: false,
            data: NsStringData {
                inline_data: inline,
            },
        }
    } else {
        let mut vec = bytes;
        vec.shrink_to_fit();
        let ptr = vec.as_mut_ptr();
        let capacity = vec.capacity();
        std::mem::forget(vec);

        NsString {
            len,
            is_heap: true,
            data: NsStringData {
                heap: ManuallyDrop::new(NsStringHeap { ptr, capacity }),
            },
        }
    }
}

// --- NEW HELPER ---
// Centralizes the client configuration so GET, POST, and any future methods
// use the exact same headers, timeouts, and settings.
fn build_default_client() -> Result<Client, NsError> {
    Client::builder()
        .user_agent(BROWSER_USER_AGENT)
        .build()
        .map_err(|_| NsError::Any)
}

/// # Safety
/// * `url` must be a valid, null-terminated C string
/// * `output` must point to a valid, zero-initialized `NsHttpResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_http_get(url: *const c_char, output: *mut NsHttpResponse) -> NsError {
    if url.is_null() || output.is_null() {
        return NsError::InvalidInput;
    }

    let url_str = unsafe { CStr::from_ptr(url).to_string_lossy() };

    // Use the central helper
    let client = match build_default_client() {
        Ok(c) => c,
        Err(e) => return e,
    };

    match client.get(url_str.as_ref()).send() {
        Ok(resp) => {
            let status = resp.status().as_u16() as c_int;
            let body_text = match resp.text() {
                Ok(t) => t,
                Err(_) => return NsError::Any,
            };

            unsafe {
                (*output).status_code = status;
                (*output).body = rust_str_to_ns(body_text);
            }

            NsError::Success
        }
        Err(_) => NsError::Any,
    }
}

/// # Safety
/// * `url` must be a valid, null-terminated C string
/// * `output` must point to a valid, zero-initialized `NsHttpResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_http_post(
    url: *const c_char,
    body_data: *const c_char,
    output: *mut NsHttpResponse,
) -> NsError {
    if url.is_null() || body_data.is_null() || output.is_null() {
        return NsError::InvalidInput;
    }

    let url_str = unsafe { CStr::from_ptr(url).to_string_lossy() };

    // Safely extract raw bytes without mutating non-UTF-8 characters
    let payload_bytes = unsafe { CStr::from_ptr(body_data).to_bytes().to_vec() };

    // Use the central helper
    let client = match build_default_client() {
        Ok(c) => c,
        Err(e) => return e,
    };

    match client.post(url_str.as_ref()).body(payload_bytes).send() {
        Ok(resp) => {
            let status = resp.status().as_u16() as c_int;
            let body_text = resp.text().unwrap_or_default();

            unsafe {
                (*output).status_code = status;
                (*output).body = rust_str_to_ns(body_text);
            }
            NsError::Success
        }
        Err(_) => NsError::Any,
    }
}

/// # Safety
/// *`resp` must be a valid pointer to an initialized `NsHttpResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_http_free(resp: *mut NsHttpResponse) {
    if resp.is_null() {
        return;
    }

    unsafe {
        ns_string_free(&mut (*resp).body);
    }
}
