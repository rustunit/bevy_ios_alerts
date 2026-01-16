#![allow(dangling_pointers_from_temporaries)]

#[cfg(target_os = "ios")]
use std::ffi::{CString, c_char};

#[cfg(target_os = "ios")]
unsafe extern "C" {
    pub fn ios_popup_message(title: *const c_char, msg: *const c_char, button: *const c_char);
    pub fn ios_popup_dialog(
        title: *const c_char,
        msg: *const c_char,
        yes: *const c_char,
        no: *const c_char,
    );
    pub fn ios_popup_input(
        title: *const c_char,
        msg: *const c_char,
        button: *const c_char,
        placeholder: *const c_char,
    );

    pub fn ios_popup_dismiss_current();
}

#[allow(unused_variables)]
pub fn popup_msg(title: &str, msg: &str, button: &str) {
    #[cfg(target_os = "ios")]
    unsafe {
        ios_popup_message(
            CString::new(title).unwrap().as_ptr(),
            CString::new(msg).unwrap().as_ptr(),
            CString::new(button).unwrap().as_ptr(),
        );
    }
}

#[allow(unused_variables)]
pub fn popup_input(title: &str, msg: &str, button: &str, placeholder: &str) {
    #[cfg(target_os = "ios")]
    unsafe {
        ios_popup_input(
            CString::new(title).unwrap().as_ptr(),
            CString::new(msg).unwrap().as_ptr(),
            CString::new(button).unwrap().as_ptr(),
            CString::new(placeholder).unwrap().as_ptr(),
        );
    }
}

#[allow(unused_variables)]
pub fn popup_dialog(title: &str, msg: &str, button_yes: &str, button_no: &str) {
    #[cfg(target_os = "ios")]
    unsafe {
        ios_popup_dialog(
            CString::new(title).unwrap().as_ptr(),
            CString::new(msg).unwrap().as_ptr(),
            CString::new(button_yes).unwrap().as_ptr(),
            CString::new(button_no).unwrap().as_ptr(),
        );
    }
}

pub fn popup_dismiss_current() {
    #[cfg(target_os = "ios")]
    unsafe {
        ios_popup_dismiss_current();
    }
}
