use ambient_api::{
    core::{
        app::components::name,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

use packages::powerwidget::{components::*, concepts::Powerwidget};

#[main]
pub fn main() {
    init_camera_quad();
    init_temp_pw();
}

fn init_temp_pw() {
    Powerwidget {
        pw_description: "This powerwidget doesn't do anything. It's just a demo.".to_string(),
        ..Powerwidget::suggested()
    }
    .make()
    .with(name(), "demo powerwidget".to_string())
    .spawn();
}

fn init_camera_quad() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient!");
}
