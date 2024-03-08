#![allow(non_snake_case)]
#![warn(missing_docs)]

//! # 🧬 Dioxus Material Symbols
//!
//! This project provides a simple but configurable component to render Google's Material Symbols in Dioxus.
//!
//! ## 🚀 How to get started
//!
//! `cargo add dioxus-material-symbols`
//!
//! This project introduces two components:
//!
//! 1. [`MaterialIconStylesheet`](MaterialIconStylesheet)
//! 2. [`MaterialIcon`](MaterialIcon)
//!
//! To be able to use the [`MaterialIcon`](MaterialIcon) component anywhere in your code, you first have to include
//! a [`MaterialIconStylesheet`](MaterialIconStylesheet) component. When you want to use the default settings,
//! just add it to your app's root component like this:
//!
//! ```no_rust
//! MaterialIconStylesheet { }
//! ```
//!
//! Have a look at the [`MaterialIconStylesheet`](MaterialIconStylesheet) docs for more options like self-hosting the icon font file.
//!
//! After that you can use the `MaterialIcon` component like you would expect it:
//!
//! ```no_rust
//! MaterialIcon { name: "settings" }
//! ```
//!
//! You can additionally specify the color and size.
//!
//! ```no_rust
//! MaterialIcon {
//!     name: "settings",
//!     size: 24,
//!     color: MaterialIconColor::Light,
//! }
//! ```
//!
//! ## 💡 Examples
//!
//! - [Button Example](https://github.com/lennartkloock/dioxus-material-symbols/blob/main/examples/button.rs)
//!
//! ## 🔗 Useful links
//!
//! - [Overview of all symbols](https://fonts.google.com/symbols?selected=Material+Symbols) (including names)
//!
//! ### Alternatives
//!
//! - [dioxus-free-symbols](https://crates.io/crates/dioxus-free-symbols) (Support for other icon packs)
//!
//! ## 📜 License
//!
//! This software is licensed under the terms of the MIT License.
//!
//! Note: All Material Symbols are licensed under the Apache License 2.0.
//!
//! &copy; 2023 Lennart Kloock

use dioxus::prelude::*;

/// Props for the [`MaterialIconStylesheet`](MaterialIconStylesheet) component
#[derive(Props, PartialEq)]
pub struct MaterialIconStylesheetProps<'a> {
    /// Variant prop for the [`MaterialIconStylesheet`](MaterialIconStylesheet) component
    ///
    /// See [`MaterialIconVariant`](MaterialIconVariant) for more information.
    #[props(default = MaterialIconVariant::Rounded)]
    pub variant: MaterialIconVariant<'a>,
}

/// Variants (also called categories) of the Material Icon font
///
/// See all variants [here](https://fonts.google.com/symbols?selected=Material+Symbols).
#[derive(PartialEq)]
pub enum MaterialIconVariant<'a> {
    /// Outlined
    Outlined,
    /// Rounded
    Rounded,
    /// Sharp
    Sharp,
    /// Self hosted font file
    ///
    /// Provide an url to a ttf or otf file.
    /// You can download the files [here](https://github.com/google/material-design-symbols/tree/master/font).
    SelfHosted(&'a str),
}

/// Stylesheet component
///
/// This component includes the Material Icon stylesheet.
/// This is required to render all Material Symbols correctly.
///
/// You can provide a variant as a prop (e.g. Rounded).
/// When you want to provide your own self-hosted font file,
/// please use [`MaterialIconVariant::SelfHosted`](MaterialIconVariant::SelfHosted) and pass the
/// file path or url to your .ttf or .otf file to it.
/// See the [button example](https://github.com/lennartkloock/dioxus-material-symbols/blob/main/examples/button.rs).
pub fn MaterialIconStylesheet<'a>(cx: Scope<'a, MaterialIconStylesheetProps<'a>>) -> Element<'a> {
    let href = match &cx.props.variant {
        MaterialIconVariant::SelfHosted(file) => {
            return cx.render(rsx!(
                style { format!(include_str!("./self-hosted-styles.css"), file) }
            ));
        }
        MaterialIconVariant::Outlined => {
            "https://fonts.googleapis.com/icon?family=Material+Symbols+Outlined"
        }
        MaterialIconVariant::Rounded => {
            "https://fonts.googleapis.com/icon?family=Material+Symbols+Rounded"
        }
        MaterialIconVariant::Sharp => {
            "https://fonts.googleapis.com/icon?family=Material+Symbols+Sharp"
        }
    };
    cx.render(rsx!(link {
        href: "{href}",
        rel: "stylesheet"
    }))
}

/// Props for the [`MaterialIcon`](MaterialIcon) component
#[derive(Props, PartialEq)]
pub struct MaterialIconProps<'a> {
    /// Name (e.g. `home`)
    ///
    /// Browse all symbols [here](https://fonts.google.com/symbols?selected=Material+Symbols).
    pub name: &'a str,
    /// Size in pixels
    ///
    /// Optional
    pub size: Option<u32>,
    /// Color
    ///
    /// Optional
    #[props(into)]
    pub color: Option<MaterialIconColor<'a>>,
}

/// Colors of Material Symbols
///
/// As described [here](https://developers.google.com/fonts/docs/material_symbols#styling_symbols_in_material_design).
#[derive(PartialEq)]
pub enum MaterialIconColor<'a> {
    /// For using symbols as black on a light background.
    Dark,
    /// For using symbols as black on a light background.
    DarkInactive,
    /// For using symbols as white on a dark background.
    Light,
    /// For using symbols as white on a dark background.
    LightInactive,
    /// Custom color, any valid CSS color
    ///
    /// E.g.: `#0000ff` or `red`
    Custom(&'a str),
}

impl<'a> From<&'a str> for MaterialIconColor<'a> {
    fn from(value: &'a str) -> Self {
        Self::Custom(value)
    }
}

impl MaterialIconColor<'_> {
    /// Converts the color to its corresponding CSS color
    pub fn to_css_color(&self) -> &str {
        match self {
            MaterialIconColor::Dark => "rgba(0, 0, 0, 0.54)",
            MaterialIconColor::DarkInactive => "rgba(0, 0, 0, 0.26)",
            MaterialIconColor::Light => "rgba(255, 255, 255, 1)",
            MaterialIconColor::LightInactive => "rgba(255, 255, 255, 0.3)",
            MaterialIconColor::Custom(c) => c,
        }
    }
}

/// Material Icon component
///
/// This component can be used to render a Material Icon.
pub fn MaterialIcon<'a>(cx: Scope<'a, MaterialIconProps<'a>>) -> Element<'a> {
    // The `font-size` attribute has to be explicitly declared as `inherit` because the stylesheet sets a default of 24px
    let css_size = cx
        .props
        .size
        .map(|s| format!("{s}px"))
        .unwrap_or_else(|| "inherit".to_string());
    let css_color = cx
        .props
        .color
        .as_ref()
        .map(|c| format!("color: {};", c.to_css_color()))
        .unwrap_or_default();
    cx.render(rsx!(
        span {
            class: "material-symbols material-symbols-outlined material-symbols-rounded material-symbols-sharp md-48",
            style: "font-size: {css_size}; {css_color} user-select: none;",
            cx.props.name
        }
    ))
}
