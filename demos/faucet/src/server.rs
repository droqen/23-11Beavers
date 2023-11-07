use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        physics::components::{
            cube_collider, dynamic, linear_velocity, physics_controlled, plane_collider,
        },
        primitives::components::{cube, quad},
        transform::components::{lookat_target, rotation, translation},
    },
    prelude::*,
};

use packages::this::{components::*, messages::*};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::splat(15.)),
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

    for rot in vec![
        Quat::from_rotation_x(0.02),
        Quat::from_rotation_y(0.02),
        Quat::from_rotation_x(-0.02),
        Quat::from_rotation_y(-0.02),
    ] {
        Entity::new()
            .with(translation(), vec3(0., 0., 0.))
            .with(plane_collider(), ())
            .with(rotation(), rot)
            .spawn();
    }

    let faucet = Entity::new()
        .with(translation(), vec3(0., 0., 12.))
        .with(faucet_flow_rate(), 0.5)
        .with(faucet_spawn_lifetime(), 8.0)
        .with(faucet_safety(), true)
        // .with(cube(), ())
        .spawn();

    {
        let faucet = faucet.clone();
        query((drip_birth())).each_frame(move |drips| {
            let now = game_time().as_secs_f32();
            let life = entity::get_component(faucet, faucet_spawn_lifetime()).unwrap();
            for (drip, birth) in drips {
                if now >= life + birth {
                    entity::despawn(drip);
                }
            }
        });
    }

    {
        let faucet = faucet.clone();
        ambient_api::core::messages::Frame::subscribe(move |_| {
            let now = game_time().as_secs_f32();
            let mut last_birth = entity::get_component(faucet, faucet_last_birth()).unwrap_or(0.);
            let mut flow_rate = entity::get_component(faucet, faucet_flow_rate()).unwrap();
            let safety = entity::get_component(faucet, faucet_safety()).unwrap();
            if safety {
                if flow_rate > 512. {
                    entity::add_component(faucet, faucet_flow_rate(), 512.);
                    flow_rate = 512.;
                }
            }
            let between_births = 1.0 / flow_rate;
            if now >= last_birth + between_births {
                while now >= last_birth + between_births {
                    last_birth += between_births;
                    let rx: f32 = (random::<f32>() - 0.5) * flow_rate.sqrt();
                    let ry: f32 = (random::<f32>() - 0.5) * flow_rate.sqrt();
                    let drip = Entity::new()
                        .with(
                            translation(),
                            entity::get_component(faucet, translation()).unwrap()
                                + vec3(rx, ry, 0.),
                        )
                        .with(rotation(), Quat::from_vec4(random::<Vec4>().normalize()))
                        .with(physics_controlled(), ())
                        .with(dynamic(), true)
                        .with(cube(), ())
                        .with(cube_collider(), Vec3::splat(1.))
                        .with(drip_birth(), now)
                        // .with(
                        //     drip_life(),
                        //     entity::get_component(faucet, faucet_spawn_lifetime()).unwrap(),
                        // )
                        .spawn();
                    physics::add_impulse(drip, vec3(0., 0., -10000.)); // doesn't work?
                }
                entity::add_component(faucet, faucet_last_birth(), last_birth);
            }
        });
    }

    {
        let faucet = faucet.clone();
        AlterFaucetFlowRate::subscribe(move |_ctx, msg| {
            if msg.direction > 0 {
                entity::mutate_component(faucet, faucet_flow_rate(), |rate| {
                    if *rate >= 256. {
                        *rate += 128.;
                    } else {
                        *rate *= 2.;
                    }
                });
            } else {
                entity::mutate_component(faucet, faucet_flow_rate(), |rate| {
                    if *rate >= 256. {
                        *rate -= 128.;
                    } else {
                        *rate /= 2.;
                    }
                });
            }
        });
    }

    {
        let faucet = faucet.clone();
        AlterFaucetLifetime::subscribe(move |_ctx, msg| {
            if msg.direction > 0 {
                entity::mutate_component(faucet, faucet_spawn_lifetime(), |rate| {
                    *rate *= 2.;
                });
            } else {
                entity::mutate_component(faucet, faucet_spawn_lifetime(), |rate| {
                    *rate /= 2.;
                });
            }
        });
    }

    {
        let faucet = faucet.clone();

        let cube_query = query(()).requires(cube()).build();

        EmergencyReset::subscribe(move |_ctx, _msg| {
            entity::add_component(faucet, faucet_flow_rate(), 0.5);
            entity::add_component(faucet, faucet_spawn_lifetime(), 8.0);
            entity::add_component(faucet, faucet_safety(), true);
            for (cubent, _) in cube_query.evaluate() {
                entity::despawn(cubent);
            }
        });
    }

    {
        let faucet = faucet.clone();

        ToggleSafety::subscribe(move |_ctx, _msg| {
            entity::mutate_component(faucet, faucet_safety(), |safety| *safety = !*safety);
        });
    }

    println!("Hello, Ambient!");
}
