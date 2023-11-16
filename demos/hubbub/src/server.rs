use ambient_api::{core::app::components::name, core::messages::Frame, prelude::*};

use packages::{
    powerwidget::{components::*, concepts::Powerwidget},
    this::{components::*, messages::TextToClient},
};
// use packages::subhub::messages::{TextFromSubhub, TextInternalSubhub, TextToSubhub};

#[main]
pub fn main() {
    init_scmsg_pw();
}

fn init_scmsg_pw() {
    let pw_rel = Powerwidget {
        pw_description: "garbage send_client_broadcast_reliable per frame".to_string(),
        ..Powerwidget::suggested()
    }
    .make()
    .with(name(), "scmsg-reliable".to_string())
    .spawn();

    let pw_un = Powerwidget {
        pw_description: "garbage send_client_broadcast_unreliable per frame".to_string(),
        ..Powerwidget::suggested()
    }
    .make()
    .with(name(), "scmsg-unreliable".to_string())
    .spawn();

    Frame::subscribe(move |_| {
        let messages_per_frame = entity::get_component(pw_rel, pw_calculated_value()).unwrap();
        for _i in 0..messages_per_frame as u32 {
            TextToClient {
                text: "Reliable Message".to_string(),
            }
            .send_client_broadcast_reliable();
        }
        let messages_per_frame = entity::get_component(pw_un, pw_calculated_value()).unwrap();
        for _i in 0..messages_per_frame as u32 {
            TextToClient {
                text: "Unreliable Message".to_string(),
            }
            .send_client_broadcast_unreliable();
        }
    });
}
