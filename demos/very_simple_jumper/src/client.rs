use ambient_api::prelude::*;

use packages::{
    pixie::components::*,
    this::{components::*, messages::*},
};

#[main]
pub fn main() {
    spawn_query(player_pixie_pos()).bind(|plrs| {
        for (plr, pos) in plrs {
            entity::add_components(
                plr,
                Entity::new()
                    .with(pix_z(), 4)
                    .with(pix_sprurl(), packages::this::assets::url("chara.png"))
                    .with(pix_place(), pos)
                    .with(pix_size(), ivec2(10, 10)),
            );
        }
    });
    query(player_pixie_pos())
        .requires(pix_place())
        .each_frame(|plrs| {
            for (plr, pos) in plrs {
                entity::set_component(plr, pix_place(), pos);
            }
        });

    ambient_api::core::messages::Frame::subscribe(|_| {
        let (delta, input) = input::get_delta();
        StickControl {
            pinx: {
                let mut pinx = 0.;
                if input.keys.contains(&KeyCode::Left) || input.keys.contains(&KeyCode::A) {
                    pinx -= 1.;
                }
                if input.keys.contains(&KeyCode::Right) || input.keys.contains(&KeyCode::D) {
                    pinx += 1.;
                }
                pinx
            },
        }
        .send_server_unreliable();
        if delta.keys.contains(&KeyCode::Z)
            || delta.keys.contains(&KeyCode::X)
            || delta.keys.contains(&KeyCode::Space)
            || delta.keys.contains(&KeyCode::Up)
        {
            JumpButton {}.send_server_reliable();
        }
    });
}
