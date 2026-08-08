#[cfg(target_os = "ios")]
mod native;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Debug, Default)]
struct NonSendRes;

/// Request a native alert popup. Every variant but [`IosAlert::Dismiss`] is answered with an
/// [`IosAlertResponse`].
#[derive(Message, Clone, Debug)]
pub enum IosAlert {
    Message {
        msg: String,
        title: String,
        button: String,
    },
    Dialog {
        msg: String,
        title: String,
        button_yes: String,
        button_no: String,
    },
    Input {
        msg: String,
        title: String,
        button: String,
        placeholder: String,
    },
    Dismiss,
}

#[derive(Clone, Debug)]
pub enum IosAlertDialogButton {
    Yes,
    No,
}

/// Sent once the user dismissed an alert requested via [`IosAlert`].
#[derive(Message, Clone, Debug)]
pub enum IosAlertResponse {
    MessageConfirm,
    Dialog(IosAlertDialogButton),
    Input(String),
}

pub struct IosAlertsPlugin;

impl Plugin for IosAlertsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<IosAlert>()
            .init_non_send_resource::<NonSendRes>()
            .add_systems(Update, process_events.run_if(on_message::<IosAlert>));

        #[cfg(not(target_os = "ios"))]
        {
            app.add_message::<IosAlertResponse>();
        }

        #[cfg(target_os = "ios")]
        {
            use bevy_channel_message::{ChannelMessageApp, ChannelMessageSender};

            app.add_channel_message::<IosAlertResponse>();

            let sender = app
                .world()
                .get_resource::<ChannelMessageSender<IosAlertResponse>>()
                .unwrap()
                .clone();

            native::set_sender(sender);
        }
    }
}

// `NonSend` to keep this on the main thread - UIKit is main-thread only.
fn process_events(mut events: MessageReader<IosAlert>, _main_thread: NonSend<NonSendRes>) {
    for e in events.read() {
        #[cfg(target_os = "ios")]
        native::show(e);
        #[cfg(not(target_os = "ios"))]
        let _ = e;
    }
}
