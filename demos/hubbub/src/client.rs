use ambient_api::core::input::messages;
use ambient_api::core::transform::components::{rotation, translation};
use ambient_api::prelude::ElementComponentExt;
use ambient_api::{
    core::messages::Frame, core::rect::components::size_from_background_image, prelude::*,
    ui::ImageFromUrl,
};

use packages::pixie::components::*;
use packages::this::{assets, components::*, messages::TextToClient};
// use packages::this::components::*;

#[main]
pub fn main() {
    let cs_msg_counter = Entity::new()
        .with(cs_msgs_rcvd(), 0)
        .with(cs_msgs_rcvd_last_frame(), 0)
        .spawn();

    {
        let cs_msg_counter = cs_msg_counter.clone();
        TextToClient::subscribe(move |_ctx, msg| {
            msg.text;
            entity::mutate_component(cs_msg_counter, cs_msgs_rcvd(), |ct| *ct += 1);
        });
    }

    {
        Entity::new()
            .with(pix_string(), "messages received each frame:".to_string())
            .with(pix_place(), ivec2(100, 90))
            .with(pix_z(), 0)
            .spawn();
        let ctr_strixie = Entity::new()
            .with(pix_string(), "> 0".to_string())
            .with(pix_place(), ivec2(100, 100))
            .with(pix_z(), 0)
            .spawn();

        const MAX_CHARS_PER_LINE: usize = 100;
        const MAX_CHARS_TOTAL: usize = 1000;

        query((cs_msgs_rcvd(), cs_msgs_rcvd_last_frame())).each_frame(move |ctrs| {
            for (ctr, (ct, ctlast)) in ctrs {
                entity::mutate_component(ctr_strixie, pix_string(), |str| {
                    let chars = str.len();
                    *str = match chars > MAX_CHARS_TOTAL {
                        // true => format!("{},\n{ct}", str[chars / 2..].to_string()), // over limit
                        true => format!("> {ct}"), // over limit
                        false => {
                            let linebreaks = str.matches("\n").count();
                            format!(
                                "{},{}{}",
                                str,
                                match chars > (1 + linebreaks) * MAX_CHARS_PER_LINE {
                                    true => "\n", // too many chars per line
                                    false => " ",
                                },
                                ct
                            )
                        }
                    };
                });
                entity::add_component(ctr, cs_msgs_rcvd_last_frame(), ct);
                entity::add_component(ctr, cs_msgs_rcvd(), 0);
            }
        });
    }

    // Entity::new()
    //     .with(pix_sprurl(), assets::url("loading_spinner_thick.png"))
    //     .with(pix_place(), ivec2(100, 100))
    //     .with(pix_size(), ivec2(40, 40))
    //     .with(pix_z(), 5)
    //     .spawn();
}
