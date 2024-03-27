<h1 align="center">
  <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example1.png" alt="Demo image 1" width="30%" height="30%">
 <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example2.png" alt="Demo image 2" width="30%" height="30%">
 <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example3.png" alt="Demo image 3" width="30%" height="30%">

M3-EGUI 

[![Stars](https://img.shields.io/github/stars/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/stargazers)
[![Forks](https://img.shields.io/github/forks/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/network/members)
[![Pull Requests](https://img.shields.io/github/issues-pr/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/pulls)
[![Issues](https://img.shields.io/github/issues/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/issues)
[![Contributors](https://img.shields.io/github/contributors/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/graphs/contributors)
[![Licence](https://img.shields.io/github/license/toastxc/material-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/material-egui/blob/main/LICENCE)
</h1>

## About
M3-EGUI (material-egui) is a library that takes an input color and generates a color palette using M3 colors. It then applies those colors to the Egui components.

## Installation
```rust
cargo install material-egui
```
## Adding to projects
### Cargo.toml
```toml
material-egui = "*"
```
### main.rs
This can be done anywhere that CTX is available.
The same can be done with Ui but this is not recommended
```rust
// input color, is_dark_theme, zoom scale
MaterialColors::new(String::from("F0F"), true, 1.5).apply(&ctx);

// if you'd like the ability to scale windows AND a better default scale, use this
MaterialColors::new(String::from("F0F"), true, 1.5).apply(&ctx, &mut self.zoom);
```

## FAQ

### What features does this have currently?
- [x] material color design
- [x] dark and light theme support
- [x] hot reloading
- [x] Egui default components
- [ ] wallpaper color picking
- [ ] M3 animations
- [ ] M3 styled components

### Will this work on my project?
Yes it will, as long as you have the following minimum versions
```toml
egui = "0.26.2"
eframe = "0.26.2"
```
### Why does (insert element here) look weird / unreadable text?
I have not tested every element combination and there is probably a few edge cases, make sure to create an issue for them to be resolved

### How do I create use the error style?
You will need a scope and the error style changer, this is a temporary solution until I find a better one
```rust
// this scope applies error colors to all elements inside
    ui.scope(|ui| {
        MaterialColors::new(String::from("F0F"), true, 1.5).error_apply(ui);
        ui.button("Error button!")
    });
// alternatively, if you already have your colors defined
    ui.scope(|ui| {
        self.colors.error_apply(ui);
        ui.button("Error button!")
    });
```

## Thanks
Special thanks to the Egui team, and the people who made `material-colors` who made this project possible.

