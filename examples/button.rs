#![allow(non_snake_case)]

//! # Button Example
//!
//! This example renders a material icon into a button which can be clicked to toggle the
//! color of the icon.

use dioxus::prelude::*;

use dioxus_material_symbols::{MaterialIcon, MaterialIconStylesheet, MaterialIconVariant};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let is_blue = use_state(&cx, || false);

    cx.render(rsx!(
        MaterialIconStylesheet {
            // Uses the self-hosted approach
            variant: MaterialIconVariant::SelfHosted("examples/assets/MaterialIcons-Regular.ttf")
        }
        button {
            style: "padding: 10; font-size: 48px;",
            onclick: move |_| is_blue.set(!is_blue),
            // The size prop was omitted, so both icons inherit their size from the button element above
            if *is_blue.get() {
                // Render material icon "home" in blue
                rsx!(MaterialIcon { name: "home", color: "blue" })
            } else {
                // Render material icon "home" in default color
                rsx!(MaterialIcon { name: "home" })
            }
        }
    ))
}
