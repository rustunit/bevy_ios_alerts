use std::{
    ffi::{c_char, c_uchar, CStr},
    sync::OnceLock,
};

use bevy::prelude::*;
use bevy_crossbeam_event::CrossbeamEventSender;

#[derive(Clone, Debug)]
pub enum IosPopupDialogButton {
    Yes,
    No,
}

#[derive(Event, Clone, Debug)]
pub enum IosPopupResponse {
    MessageConfirm,
    Dialog(IosPopupDialogButton),
    Input(String),
}

static SENDER: OnceLock<Option<CrossbeamEventSender<IosPopupResponse>>> = OnceLock::new();

pub fn set_sender(sender: CrossbeamEventSender<IosPopupResponse>) {
    while !SENDER.set(Some(sender.clone())).is_ok() {}
}

#[no_mangle]
pub unsafe extern "C" fn popup_message_click() {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosPopupResponse::MessageConfirm);
}
#[no_mangle]
pub unsafe extern "C" fn popup_dialog_click(button: c_uchar) {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosPopupResponse::Dialog(if button == 0 {
            IosPopupDialogButton::Yes
        } else {
            IosPopupDialogButton::No
        }));
}

#[no_mangle]
pub unsafe extern "C" fn popup_input_click(text: *const c_char) {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosPopupResponse::Input(
            CStr::from_ptr(text).to_str().unwrap().to_string(),
        ));
}
