#[cfg(target_os = "ios")]
use std::{
    ffi::{c_char, c_uchar, CStr},
    sync::OnceLock,
};

use bevy::prelude::*;

#[cfg(target_os = "ios")]
use bevy_channel_message::ChannelMessageSender;

#[derive(Clone, Debug)]
pub enum IosAlertDialogButton {
    Yes,
    No,
}

#[derive(Message, Clone, Debug)]
pub enum IosAlertResponse {
    MessageConfirm,
    Dialog(IosAlertDialogButton),
    Input(String),
}

#[cfg(target_os = "ios")]
static SENDER: OnceLock<Option<ChannelMessageSender<IosAlertResponse>>> = OnceLock::new();

#[cfg(target_os = "ios")]
pub fn set_sender(sender: ChannelMessageSender<IosAlertResponse>) {
    while !SENDER.set(Some(sender.clone())).is_ok() {}
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn popup_message_click() {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosAlertResponse::MessageConfirm);
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn popup_dialog_click(button: c_uchar) {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosAlertResponse::Dialog(if button == 0 {
            IosAlertDialogButton::Yes
        } else {
            IosAlertDialogButton::No
        }));
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn popup_input_click(text: *const c_char) {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosAlertResponse::Input(
            CStr::from_ptr(text).to_str().unwrap().to_string(),
        ));
}
