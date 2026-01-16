mod native;
mod native_events;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub use native_events::{IosAlertDialogButton, IosAlertResponse};

#[derive(Resource, Clone, Debug, Default)]
struct NonSendRes;

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

            native_events::set_sender(sender);
        }
    }
}

fn process_events(mut events: MessageReader<IosAlert>, _main_thread: NonSend<NonSendRes>) {
    while let Some(e) = events.read().next() {
        match e {
            IosAlert::Message { msg, title, button } => native::popup_msg(title, msg, button),
            IosAlert::Dialog {
                msg,
                title,
                button_yes,
                button_no,
            } => native::popup_dialog(title, msg, button_yes, button_no),
            IosAlert::Input {
                msg,
                title,
                button,
                placeholder,
            } => native::popup_input(title, msg, button, placeholder),
            IosAlert::Dismiss => native::popup_dismiss_current(),
        }
    }
}
