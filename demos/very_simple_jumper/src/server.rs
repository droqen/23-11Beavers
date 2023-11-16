use ambient_api::{core::player::components::is_player, prelude::*};

use packages::{
    jellybean::components::*,
    this::{components::*, messages::*},
};

#[main]
pub fn main() {
    // floor
    Entity::new()
        .with(jellybean_pos(), ivec2(0, 100))
        .with(jellybean_hitbox(), ivec4(0, 0, 200, 10))
        .with(jellybean_is_solid(), ())
        .spawn();

    spawn_query(()).requires(is_player()).bind(|plrs| {
        for (plr, _) in plrs {
            entity::add_components(
                plr,
                Entity::new()
                    .with(jellybean_pos(), ivec2(50, 50))
                    .with(jellybean_subpos(), Vec2::ZERO)
                    .with(jellybean_hitbox(), ivec4(0, 0, 10, 10))
                    .with(jellybean_vel(), vec2(0., 0.))
                    .with(jellybean_casts_down(), ())
                    .with(jellybean_touching_down(), false)
                    .with(pinx(), 0.)
                    .with(jumpbuf(), 0.)
                    .with(florbuf(), 0.)
                    .with(player_pixie_pos(), ivec2(100 - 5, 75 - 5)),
            );
        }
    });
    query((
        pinx(),
        jumpbuf(),
        florbuf(),
        jellybean_pos(),
        jellybean_touching_down(),
    ))
    .each_frame(|plrs| {
        let dt = delta_time();
        for (plr, (pinput_x, jbuf, mut fbuf, pos, is_floored)) in plrs {
            if is_floored {
                fbuf = 0.1;
            }

            {
                let dt = dt.clone();
                entity::mutate_component(plr, jellybean_vel(), move |vel| {
                    vel.x = tow(vel.x, 1. * pinput_x, 2. * dt);
                    vel.y = tow(vel.y, 3., 7. * dt);
                    println!("Players pos={pos:?} Players vel={vel:?}");
                });
            }

            if jbuf > 0. && fbuf > 0. {
                // do a jump.
                entity::set_component(plr, jumpbuf(), 0.);
                entity::set_component(plr, florbuf(), 0.);
                entity::mutate_component(plr, jellybean_vel(), |vel| {
                    if vel.y >= 0. {
                        vel.y = -1.5;
                    }
                });
            } else {
                // reduce bufs.
                entity::set_component(plr, jumpbuf(), (jbuf - dt).max(0.));
                entity::set_component(plr, florbuf(), (fbuf - dt).max(0.));
            }

            entity::add_component(plr, player_pixie_pos(), pos);
        }
    });

    StickControl::subscribe(|ctx, msg| {
        if let Some(plr) = ctx.client_entity_id() {
            entity::add_component(plr, pinx(), msg.pinx);
        }
    });

    JumpButton::subscribe(|ctx, _msg| {
        if let Some(plr) = ctx.client_entity_id() {
            entity::add_component(plr, jumpbuf(), 0.1);
        }
    });
}

fn tow(a: f32, b: f32, rate: f32) -> f32 {
    if a + rate < b {
        a + rate
    } else if a - rate > b {
        a - rate
    } else {
        b
    }
}
