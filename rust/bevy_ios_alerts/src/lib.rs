mod native;

mod native_events;

use bevy::prelude::*;

pub use native_events::IosAlertResponse;

#[derive(Resource, Clone, Debug, Default)]
struct NonSendRes;

#[derive(Event, Clone, Debug)]
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
        app.add_event::<IosAlert>()
            .init_non_send_resource::<NonSendRes>()
            .add_systems(Update, process_events.run_if(on_event::<IosAlert>()));

        #[cfg(not(target_os = "ios"))]
        {
            app.add_event::<IosAlertResponse>();
        }

        #[cfg(target_os = "ios")]
        {
            use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

            app.add_crossbeam_event::<IosAlertResponse>();

            let sender = app
                .world
                .get_resource::<CrossbeamEventSender<IosAlertResponse>>()
                .unwrap()
                .clone();

            native_events::set_sender(sender);
        }
    }
}

fn process_events(mut events: EventReader<IosAlert>, _main_thread: NonSend<NonSendRes>) {
    while let Some(e) = events.read().next() {
        match e {
            IosAlert::Message { msg, title, button } => native::popup_msg(msg, title, button),
            IosAlert::Dialog {
                msg,
                title,
                button_yes,
                button_no,
            } => native::popup_dialog(msg, title, button_yes, button_no),
            IosAlert::Input {
                msg,
                title,
                button,
                placeholder,
            } => native::popup_input(msg, title, button, placeholder),
            IosAlert::Dismiss => native::popup_dismiss_current(),
        }
    }
}
