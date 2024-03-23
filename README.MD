<h1 align="center">
  <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example1.png" alt="Demo image 1" width="30%" height="30%">
 <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example2.png" alt="Demo image 2" width="30%" height="30%">
 <img src="https://github.com/toastxc/material3-egui/blob/main/README_RESOURCES/example3.png" alt="Demo image 3" width="30%" height="30%">

M3-EGUI 

[![Stars](https://img.shields.io/github/stars/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/stargazers)
[![Forks](https://img.shields.io/github/forks/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/network/members)
[![Pull Requests](https://img.shields.io/github/issues-pr/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/pulls)
[![Issues](https://img.shields.io/github/issues/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/issues)
[![Contributors](https://img.shields.io/github/contributors/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/graphs/contributors)
[![Licence](https://img.shields.io/github/license/toastxc/material3-egui?style=flat-square&logoColor=white)](https://github.com/toastxc/Reywen-Revolt/blob/main/LICENCE)
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
This can be done anywhere that CTX is available
```rust
// input color, is_dark_theme, zoom scale
material_egui::apply(String::from("F0F"), true, 1.5, &ctx);
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


## Thanks
Special thanks to the Egui team, and the people who made `material-colors` who made this project possible.

