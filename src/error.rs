use std::cell::RefCell;
use std::ffi::CString;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
    static ERROR_NUM: RefCell<i32> = const { RefCell::new(0) };
}

pub fn set_error(msg: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::new(msg).ok();
    });
    ERROR_NUM.with(|n| {
        *n.borrow_mut() += 1;
    });
}

pub fn clear_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
}

pub fn get_error_ptr() -> *const i8 {
    LAST_ERROR.with(|e| {
        e.borrow()
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(std::ptr::null())
    })
}

pub fn get_errno() -> i32 {
    ERROR_NUM.with(|n| *n.borrow())
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_errno() -> i32 {
    let errno = get_errno();
    ERROR_NUM.with(|n| *n.borrow_mut() = 0);
    errno
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_errmsg(error: i32) -> *const i8 {
    static UNKNOWN: &[u8] = b"unknown error\0";
    static NO_ERROR: &[u8] = b"no error\0";

    if error == 0 {
        let ptr = get_error_ptr();
        if ptr.is_null() {
            return NO_ERROR.as_ptr() as *const i8;
        }
        return ptr;
    }

    if error == -1 {
        let ptr = get_error_ptr();
        if !ptr.is_null() {
            return ptr;
        }
    }

    UNKNOWN.as_ptr() as *const i8
}
