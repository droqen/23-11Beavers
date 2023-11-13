use ambient_api::{
    core::{app::components::name, rendering::components::color, text::components::font_size},
    prelude::*,
};

use packages::this::{components::*, messages::AdjustPower};

#[main]
pub fn main() {
    PowerwidgetsApp::el().spawn_interactive();
}

#[element_component]
#[allow(non_snake_case)]
fn PowerwidgetsApp(hooks: &mut Hooks) -> Element {
    let pws = ambient_element::use_query(
        hooks,
        (
            name(),
            pw_base(),
            pw_exponent(),
            pw_calculated_value(),
            pw_max_value(),
            pw_description(),
        ),
    );

    const PW_FONT_SIZE: f32 = 20.;

    FlowColumn::el(
        pws.into_iter()
            .map(|(_pw, (nom, base, exp, val, maxval, desc))| {
                let overlimit = val >= maxval && maxval > 0.;
                FlowColumn::el([
                    Text::el(format!("{nom}")).with(font_size(), PW_FONT_SIZE),
                    Text::el(format!("{desc}")),
                    FlowRow::el([
                        Text::el(".....").with(color(), Vec4::ZERO), // tab
                        {
                            let msg = AdjustPower {
                                widget_name: nom.clone(),
                                set_base: base - 1,
                                set_exponent: exp,
                            };
                            Button::new("-", move |_| {
                                msg.send_server_reliable();
                            })
                            .disabled(base - 1 < 2)
                            .style(ButtonStyle::Regular)
                            .tooltip(Text::el("Decrease base by 1 (min 2)"))
                            .el()
                        },
                        {
                            let msg = AdjustPower {
                                widget_name: nom.clone(),
                                set_base: base + 1,
                                set_exponent: exp,
                            };
                            Button::new("+", move |_| {
                                msg.send_server_reliable();
                            })
                            .disabled(overlimit)
                            .style(ButtonStyle::Regular)
                            .tooltip(Text::el("Increase base by 1"))
                            .el()
                        },
                        Text::el(format!(" ")).with(font_size(), PW_FONT_SIZE),
                        Text::el(format!("{base}"))
                            .with(color(), Vec4::splat(1.))
                            .with(font_size(), PW_FONT_SIZE),
                        Text::el(format!("{exp}")).with(color(), Vec4::splat(1.)),
                        Text::el(format!(" ")).with(font_size(), PW_FONT_SIZE),
                        {
                            let msg = AdjustPower {
                                widget_name: nom.clone(),
                                set_base: base,
                                set_exponent: exp - 1,
                            };
                            Button::new("-", move |_| {
                                msg.send_server_reliable();
                            })
                            .disabled(exp - 1 < 2)
                            .style(ButtonStyle::Regular)
                            .tooltip(Text::el("Decrease exponent by 1 (min 2)"))
                            .el()
                        },
                        {
                            let msg = AdjustPower {
                                widget_name: nom.clone(),
                                set_base: base,
                                set_exponent: exp + 1,
                            };
                            Button::new("+", move |_| {
                                msg.send_server_reliable();
                            })
                            .disabled(overlimit)
                            .style(ButtonStyle::Regular)
                            .tooltip(Text::el("Increase exponent by 1"))
                            .el()
                        },
                        Text::el(" = ").with(font_size(), PW_FONT_SIZE),
                        Text::el(format!("{val}"))
                            .with(color(), Vec4::splat(1.))
                            .with(font_size(), PW_FONT_SIZE),
                        Text::el(
                            match overlimit {
                                false => "",
                                true => " (MAX!)",
                            }
                            .to_string(),
                        ),
                    ])
                    .with_padding_even(2.),
                ])
            }),
    )
    .with_background(vec4(0., 0., 0., 0.9))
    .with_padding_even(10.)
}
