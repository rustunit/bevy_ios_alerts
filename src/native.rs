//! Direct UIKit access via [`objc2`].
//!
//! Alerts are presented from the Bevy schedule (main thread), while the answer arrives later from
//! the UIKit action handler - so responses are pushed into the `bevy_channel_message` channel and
//! picked up by the schedule from there.

use std::cell::RefCell;
use std::ptr::NonNull;
use std::sync::OnceLock;

use bevy_channel_message::ChannelMessageSender;
use block2::RcBlock;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_foundation::NSString;
use objc2_ui_kit::{
    UIAlertAction, UIAlertActionStyle, UIAlertController, UIAlertControllerStyle, UIApplication,
    UISceneActivationState, UITextField, UIViewController, UIWindowScene,
};

use crate::{IosAlert, IosAlertDialogButton, IosAlertResponse};

static SENDER: OnceLock<ChannelMessageSender<IosAlertResponse>> = OnceLock::new();

pub fn set_sender(sender: ChannelMessageSender<IosAlertResponse>) {
    let _ = SENDER.set(sender);
}

/// A response can arrive before the plugin was built; dropping it is the only option, since
/// unwinding out of an Objective-C block would be undefined behavior.
fn send(msg: IosAlertResponse) {
    let Some(sender) = SENDER.get() else {
        bevy_log::warn!("alert response dropped: plugin not initialized");
        return;
    };
    sender.send(msg);
}

thread_local! {
    /// Only reachable to dismiss it again - UIKit owns the presented alert. Thread local because
    /// [`Retained`] is neither `Send` nor `Sync` and every access happens on the main thread.
    static CURRENT: RefCell<Option<Retained<UIAlertController>>> = const { RefCell::new(None) };
}

pub fn show(alert: &IosAlert) {
    let Some(mtm) = MainThreadMarker::new() else {
        bevy_log::error!("alerts: alert requested off the main thread");
        return;
    };

    match alert {
        IosAlert::Message { msg, title, button } => popup_message(mtm, title, msg, button),
        IosAlert::Dialog {
            msg,
            title,
            button_yes,
            button_no,
        } => popup_dialog(mtm, title, msg, button_yes, button_no),
        IosAlert::Input {
            msg,
            title,
            button,
            placeholder,
        } => popup_input(mtm, title, msg, button, placeholder),
        IosAlert::Dismiss => dismiss_current(),
    }
}

fn popup_message(mtm: MainThreadMarker, title: &str, msg: &str, button: &str) {
    let alert = new_alert(mtm, title, msg);

    let handler = RcBlock::new(move |_action: NonNull<UIAlertAction>| {
        forget_current();
        send(IosAlertResponse::MessageConfirm);
    });
    alert.addAction(&action(mtm, button, &handler));

    present(mtm, alert);
}

fn popup_dialog(mtm: MainThreadMarker, title: &str, msg: &str, yes: &str, no: &str) {
    let alert = new_alert(mtm, title, msg);

    for (button, answer) in [
        (yes, IosAlertDialogButton::Yes),
        (no, IosAlertDialogButton::No),
    ] {
        let handler = RcBlock::new(move |_action: NonNull<UIAlertAction>| {
            forget_current();
            send(IosAlertResponse::Dialog(answer.clone()));
        });
        alert.addAction(&action(mtm, button, &handler));
    }

    present(mtm, alert);
}

fn popup_input(mtm: MainThreadMarker, title: &str, msg: &str, button: &str, placeholder: &str) {
    let alert = new_alert(mtm, title, msg);

    let placeholder = NSString::from_str(placeholder);
    let configure = RcBlock::new(move |field: NonNull<UITextField>| {
        // SAFETY: UIKit hands us the text field it just created for this alert.
        unsafe { field.as_ref() }.setPlaceholder(Some(&placeholder));
    });
    alert.addTextFieldWithConfigurationHandler(Some(&configure));

    // Capturing the field instead of the alert avoids a retain cycle: the alert owns the action,
    // the action owns this block, and the field owns nothing of the two.
    let field = alert.textFields().and_then(|fields| fields.firstObject());
    let handler = RcBlock::new(move |_action: NonNull<UIAlertAction>| {
        forget_current();
        let text = field
            .as_ref()
            .and_then(|field| field.text())
            .map(|text| text.to_string())
            .unwrap_or_default();
        send(IosAlertResponse::Input(text));
    });
    alert.addAction(&action(mtm, button, &handler));

    present(mtm, alert);
}

fn dismiss_current() {
    if let Some(alert) = CURRENT.take() {
        alert.dismissViewControllerAnimated_completion(false, None);
    }
}

/// UIKit dismisses the alert itself once an action fired, we only drop our handle on it.
fn forget_current() {
    CURRENT.with_borrow_mut(|current| *current = None);
}

fn new_alert(mtm: MainThreadMarker, title: &str, msg: &str) -> Retained<UIAlertController> {
    UIAlertController::alertControllerWithTitle_message_preferredStyle(
        Some(&NSString::from_str(title)),
        Some(&NSString::from_str(msg)),
        UIAlertControllerStyle::Alert,
        mtm,
    )
}

fn action(
    mtm: MainThreadMarker,
    title: &str,
    handler: &block2::DynBlock<dyn Fn(NonNull<UIAlertAction>)>,
) -> Retained<UIAlertAction> {
    UIAlertAction::actionWithTitle_style_handler(
        Some(&NSString::from_str(title)),
        UIAlertActionStyle::Default,
        Some(handler),
        mtm,
    )
}

fn present(mtm: MainThreadMarker, alert: Retained<UIAlertController>) {
    let Some(root) = root_view_controller(mtm) else {
        bevy_log::error!("alerts: found no root view controller to present the alert on");
        return;
    };

    root.presentViewController_animated_completion(&alert, true, None);

    CURRENT.with_borrow_mut(|current| *current = Some(alert));
}

/// `UIApplication::keyWindow` is deprecated, so walk the connected scenes instead.
fn root_view_controller(mtm: MainThreadMarker) -> Option<Retained<UIViewController>> {
    let mut background = None;

    for scene in UIApplication::sharedApplication(mtm)
        .connectedScenes()
        .iter()
    {
        let Some(scene) = scene.downcast_ref::<UIWindowScene>() else {
            continue;
        };

        let root = scene
            .keyWindow()
            .and_then(|window| window.rootViewController())
            .or_else(|| {
                scene
                    .windows()
                    .iter()
                    .find_map(|window| window.rootViewController())
            });

        let Some(root) = root else { continue };

        if scene.activationState() == UISceneActivationState::ForegroundActive {
            return Some(root);
        }
        background.get_or_insert(root);
    }

    background
}
