use ambient_api::core::primitives::components::cube;
use ambient_api::core::rendering::components::color;
use ambient_api::prelude::*;
use packages::this::components::*;
use packages::this::messages::*;

#[main]
fn main() {
    let cube_counter = Entity::new().with(cube_count(), 0).spawn();
    spawn_query((faucet_flow_rate(), faucet_spawn_lifetime())).bind(move |faucets| {
        for (faucet, _) in faucets {
            FaucetControlApp::el(faucet, cube_counter.clone()).spawn_interactive();
        }
    });
    let cube_query = query(()).requires(cube()).build();
    ambient_api::core::messages::Frame::subscribe(move |_| {
        let (delta, input) = input::get_delta();
        if delta.keys.contains(&KeyCode::F) {
            AlterFaucetFlowRate { direction: -1 }.send_server_reliable();
        }
        if delta.keys.contains(&KeyCode::R) {
            AlterFaucetFlowRate { direction: 1 }.send_server_reliable();
        }
        if delta.keys.contains(&KeyCode::G) {
            AlterFaucetLifetime { direction: -1 }.send_server_reliable();
        }
        if delta.keys.contains(&KeyCode::T) {
            AlterFaucetLifetime { direction: 1 }.send_server_reliable();
        }
        if input.keys.contains(&KeyCode::Escape) {
            EmergencyReset { reset: true }.send_server_unreliable();
        }
        if input.keys.contains(&KeyCode::LShift) && delta.keys.contains(&KeyCode::L) {
            ToggleSafety { toggle: true }.send_server_reliable();
        }

        entity::add_component(
            cube_counter,
            cube_count(),
            cube_query.evaluate().len().try_into().unwrap(),
        );
    });
}

#[element_component]
#[allow(non_snake_case)]
fn FaucetControlApp(hooks: &mut Hooks, faucet: EntityId, ccounter: EntityId) -> Element {
    let faucet_flow: f32 =
        ambient_api::element::use_entity_component(hooks, faucet, faucet_flow_rate()).unwrap();
    let faucet_slife: f32 =
        ambient_api::element::use_entity_component(hooks, faucet, faucet_spawn_lifetime()).unwrap();
    let faucet_safety: bool =
        ambient_api::element::use_entity_component(hooks, faucet, faucet_safety()).unwrap();
    let ccount: u32 =
        ambient_api::element::use_entity_component(hooks, ccounter, cube_count()).unwrap();

    FlowColumn::el([
        Text::el("FAUCET"),
        Text::el("-"),
        Text::el(format!("Faucet flow rate: {faucet_flow} per second")),
        match faucet_safety {
            false => Text::el("   Rate limit: NONE.").with(color(), Vec3::splat(1.).extend(1.)),
            true => Text::el("   Rate limit: 512.").with(color(), Vec3::splat(0.6).extend(1.)),
        },
        match (faucet_flow >= 256.) {
            true => Text::el("   +/- 128 using F<-->R").with(color(), Vec3::splat(1.).extend(1.)),
            false => {
                Text::el("   2x up/down using F<-->R").with(color(), Vec3::splat(1.).extend(1.))
            }
        },
        Text::el("-"),
        Text::el(format!("Faucet spawn_lifetime: {faucet_slife}")),
        Text::el("   2x up/down using G<-->T").with(color(), Vec3::splat(1.).extend(1.)),
        Text::el("-"),
        Text::el(format!(
            "Cube count: {ccount} / {}",
            faucet_flow * faucet_slife
        )),
        Text::el("Hold ESC for emergency reset"),
        Text::el("-"),
    ])
    .with_background(vec4(0., 0., 0., 0.9))
    .with_padding_even(10.)
}
