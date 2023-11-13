use ambient_api::{core::app::components::name, prelude::*};

use packages::this::{components::*, concepts::*, messages::AdjustPower};

#[main]
pub fn main() {
    // test1_create_widgets();

    change_query((pw_base(), pw_exponent(), pw_max_value()))
        .track_change((pw_base(), pw_exponent(), pw_max_value()))
        .bind(|pws| {
            for (pw, (base, exp, max)) in pws {
                entity::add_component(
                    pw,
                    pw_calculated_value(),
                    match max > 0. {
                        true => (base as f32).powf(exp as f32).min(max),
                        false => (base as f32).powf(exp as f32), // no limit
                    },
                );
            }
        });
    let find_pws = query(name()).requires((pw_base(), pw_exponent())).build();
    AdjustPower::subscribe(move |_ctx, msg| {
        let mut named_widget: Option<EntityId> = None;
        for (pw, pw_name) in find_pws.evaluate() {
            if pw_name == msg.widget_name {
                named_widget = Some(pw);
                break;
            }
        }
        match named_widget {
            None => {
                println!(
                    "ERR ~ Bad AdjustPower msg; could not find powerwidget with name {}",
                    msg.widget_name
                );
            }
            Some(pw) => {
                entity::set_component(pw, pw_base(), msg.set_base);
                entity::set_component(pw, pw_exponent(), msg.set_exponent);
            }
        }
    });
}

fn test1_create_widgets() {
    Powerwidget {
        ..Powerwidget::suggested()
    }
    .make()
    .with(name(), "Test1 Powerwidget A".to_string())
    .spawn();
    Powerwidget {
        ..Powerwidget::suggested()
    }
    .make()
    .with(name(), "Test1 Powerwidget B".to_string())
    .spawn();
}
