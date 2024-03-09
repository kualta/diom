# Dioxus Material Symbols

This project provides a simple but configurable component to render Google's Material Symbols in Dioxus.

It's a fork of [`dioxus-material-icons`](https://github.com/lennartkloock/dioxus-material-icons), the differences between this and cousin packages:
- More icons (3200 vs 1500)
- Fewer styles (3 vs 5)

Material symbols also supports four adjustable variable font styles (fill, weight, grade, and optical size), but they are not implemented yet.

More information and overview of all icons: [Google Material Symbols](https://fonts.google.com/icons) 

## How to get started

`cargo add dioxus-material-symbols`

This project introduces two components:

1. `MaterialIconStylesheet`
2. `MaterialIcon`

To be able to use the `MaterialIcon` component anywhere in your code, you first have to include
a `MaterialIconStylesheet` component. When you want to use the default settings, just add it to your app's root
component like this:

```
MaterialIconStylesheet { }
```

Have a look at the docs for more options like self-hosting the icon font file.

After that you can use the `MaterialIcon` component like you would expect it:

```
MaterialIcon { name: "settings" }
```

You can additionally specify the color and size.

```
MaterialIcon {
    name: "settings",
    size: 24,
    color: MaterialIconColor::Light,
}
```

## Alternatives

- [dioxus-free-icons](https://crates.io/crates/dioxus-free-icons) (Support for other icon packs)
- [dioxus-material-icons](https://crates.io/crates/dioxus-material-icons) (Support for more styles)

## License

This software is licensed under the terms of the MIT License.

Note: All Material Icons are licensed under the Apache License 2.0.

&copy; 2024 Lennart Kloock, kualta
