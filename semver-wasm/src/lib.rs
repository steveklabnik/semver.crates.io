extern crate semver;

use semver::Version;
use semver::VersionReq;

use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

macro_rules! check {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(_) => return false,
    })
}

#[no_mangle]
pub extern fn is_match(v: *mut c_char, r: *mut c_char) -> bool {
    unsafe {
        let v = CStr::from_ptr(v);
        let v = check!(v.to_str());

        let r = CStr::from_ptr(r);
        let r = check!(r.to_str());
        
        let v = check!(Version::parse(v));

        let r = check!(VersionReq::parse(r));

        r.matches(&v)
    }
}

// magic wasm shenanigans

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf); // This is JS' responsibility now
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use super::is_match;

    #[test]
    fn matches() {
        // these are going to leak but it's two tests so I don't care
        let v = CString::new("1.0.0").unwrap().into_raw();
        let r = CString::new(">= 1.0.0").unwrap().into_raw();

        assert!(is_match(v, r));
    }

    #[test]
    fn does_not_match() {
        // these are going to leak but it's two tests so I don't care
        let v = CString::new("0.1.0").unwrap().into_raw();
        let r = CString::new(">= 1.0.0").unwrap().into_raw();

        assert!(!is_match(v, r));
    }
}

