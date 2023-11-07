use ambient_api::core::transform::components::{rotation, translation};
use ambient_api::prelude::ElementComponentExt;
use ambient_api::{
    core::rect::components::size_from_background_image, prelude::*, ui::ImageFromUrl,
};

use packages::this::assets;
use packages::this::components::*;

#[main]
pub fn main() {
    let animtimeent = Entity::new().with(ui_anim_time(), 0.0).spawn();
    HubbubApp::el(animtimeent).spawn_interactive();

    let find_sents = query((sent_time(), sent_text())).build();
    ambient_api::core::messages::Frame::subscribe(move |_| {
        if random::<f32>() < 0.01 {
            Entity::new()
                .with(sent_time(), game_time().as_secs_f32())
                .with(sent_text(), "Hello World".into())
                .spawn();
        }
        if random::<f32>() < 0.01 {
            for (sent, (time, _text)) in find_sents.evaluate() {
                if time + random::<f32>() * 4.0 > game_time().as_secs_f32() {
                    entity::despawn(sent);
                }
            }
        }
    });
}

#[element_component]
#[allow(non_snake_case)]
fn HubbubApp(hooks: &mut Hooks, _animtimeent: EntityId) -> Element {
    ambient_element::use_interval(hooks, 0.25, || {});
    let prog = match (game_time().as_secs_f32() / 0.25 % 3.00).floor() as u32 {
        0 => ".",
        1 => "..",
        2 => "...",
        _ => "?",
    }
    .to_string();
    let sents = ambient_element::use_query(hooks, (sent_time(), sent_text()));
    // for (sent, (time,text))in sents {}
    FlowColumn::el(
        sents
            .into_iter()
            .map(|(sent, (time, text))| Text::el(format!("{text} @ {time} {prog}"))),
    )
    .with_background(vec4(0., 0., 0., 0.9))
    .with_padding_even(10.)

    // Group::el([
    //     LayoutFreeCenter::el(
    //         ImageFromUrl {
    //             url: assets::url("loading_spinner_thick.png"),
    //         }
    //         .el()
    //         .with(width(), 15.)
    //         .with(height(), 15.),
    //         true,
    //         true,
    //     )
    //     .with(translation(), vec3(10., 10., 0.))
    //     .with(width(), 15.)
    //     .with(height(), 15.),
    //     LayoutFreeCenter::el(
    //         ImageFromUrl {
    //             url: assets::url("loading_spinner_thick.png"),
    //         }
    //         .el()
    //         .with(width(), 15.)
    //         .with(height(), 15.),
    //         true,
    //         true,
    //     )
    //     .with(translation(), vec3(10., 30., 0.))
    //     .with(rotation(), Quat::from_rotation_z(1.0))
    //     .with(width(), 15.)
    //     .with(height(), 15.),
    //     LayoutFreeCenter::el(
    //         ImageFromUrl {
    //             url: assets::url("loading_spinner_thick.png"),
    //         }
    //         .el()
    //         .with(width(), 15.)
    //         .with(height(), 15.),
    //         true,
    //         true,
    //     )
    //     .with(translation(), vec3(10., 50., 0.))
    //     .with(width(), 15.)
    //     .with(height(), 15.),
    // ])
    // // FlowColumn::el([FlowRow::el([
    // //     Text::el("Loading"),
    // //     ImageFromUrl {
    // //         url: assets::url("loading_spinner_thick.png"),
    // //     }
    // //     .el()
    // //     .with(width(), 15.)
    // //     .with(height(), 15.)
    // //     .with(rotation(), Quat::from_rotation_z(1.0)),
    // // ])])
    // // .with_background(vec4(0., 0., 0., 0.9))
    // // .with_padding_even(10.)
}
