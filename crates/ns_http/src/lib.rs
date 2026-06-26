use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int};

use ns_error::NsError;
use ns_string::{NsString, NsStringData, NsStringHeap, ns_string_free};
use reqwest::blocking::Client;

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

/// # Safety
/// * `url` must be a vald, null-terminated C string
/// * `output` must point to a valid, zero-initialized `NsHttpResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_http_get(url: *const c_char, output: *mut NsHttpResponse) -> NsError {
    if url.is_null() || output.is_null() {
        return NsError::InvalidInput;
    }

    let url_str = unsafe { CStr::from_ptr(url).to_string_lossy() };

    match reqwest::blocking::get(url_str.as_ref()) {
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
/// * `url` must be a vald, null-terminated C string
/// * `output` must point to a valid, zero-initialized `NsHttpResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_http_post(
    url: *const c_char,
    body_data: *const c_char,
    output: *mut NsHttpResponse,
) -> NsError {
    if url.is_null() || body_data.is_null() || output.is_null() {
        return NsError::Any;
    }

    let url_str = unsafe { CStr::from_ptr(url).to_string_lossy() };
    let payload_str = unsafe { CStr::from_ptr(body_data).to_string_lossy() };

    let client = Client::new();
    match client
        .post(url_str.as_ref())
        .body(payload_str.into_owned())
        .send()
    {
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
