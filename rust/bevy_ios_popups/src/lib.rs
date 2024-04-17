mod native;
mod native_events;

use bevy::prelude::*;

use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};
use native_events::set_sender;

pub use native_events::IosPopupResponse;

#[derive(Resource, Clone, Debug, Default)]
struct NonSendRes;

#[derive(Event, Clone, Debug)]
pub enum IosPopup {
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

pub struct IosPopupsPlugin;

impl Plugin for IosPopupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IosPopup>()
            .init_non_send_resource::<NonSendRes>()
            .add_crossbeam_event::<IosPopupResponse>()
            .add_systems(Update, process_events.run_if(on_event::<IosPopup>()));

        let sender = app
            .world
            .get_resource::<CrossbeamEventSender<IosPopupResponse>>()
            .unwrap()
            .clone();

        set_sender(sender);
    }
}

fn process_events(mut events: EventReader<IosPopup>, _main_thread: NonSend<NonSendRes>) {
    while let Some(e) = events.read().next() {
        match e {
            IosPopup::Message { msg, title, button } => native::popup_msg(msg, title, button),
            IosPopup::Dialog {
                msg,
                title,
                button_yes,
                button_no,
            } => native::popup_dialog(msg, title, button_yes, button_no),
            IosPopup::Input {
                msg,
                title,
                button,
                placeholder,
            } => native::popup_input(msg, title, button, placeholder),
            IosPopup::Dismiss => native::popup_dismiss_current(),
        }
    }
}
