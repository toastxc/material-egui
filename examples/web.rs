use dotenv::dotenv;
use eframe::Frame;
use egui::{vec2, CollapsingHeader, Context, Slider, Ui, Widget};
use material_colors::Argb;
use material_egui::MaterialColors;
use std::env;
use std::str::FromStr;

static MIN_WIDTH: f32 = 300.0;
static DEFAULT_WIDTH: f32 = 480.0;
static MIN_HEIGHT: f32 = 480.0;
static DEFAULT_HEIGHT: f32 = 480.0;

fn main() {
    dotenv().ok();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT))
            .with_min_inner_size([MIN_WIDTH, MIN_HEIGHT])
            .with_transparent(true),
        vsync: true,
        follow_system_theme: true,
        centered: false,
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    eframe::run_native("App", options, Box::new(|_cc| Box::from(App::default()))).unwrap();
}

#[derive(Debug, Clone)]
struct App {
    base_color: String,
    edit_base_color: String,
    dark_theme: bool,
    enabled: bool,
    options_open: bool,
    first_run: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            base_color: env::var("BASE_COLOR").unwrap(),
            edit_base_color: env::var("BASE_COLOR").unwrap(),
            dark_theme: env::var("DARK_THEME").unwrap().parse().unwrap(),
            enabled: true,
            options_open: true,
            first_run: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        MaterialColors::new(self.base_color.clone(), self.dark_theme, 1.5)
            .apply_zoom(ctx, &mut self.first_run);

        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }
}

fn update_fn(value: &mut App, ui: &mut Ui) {
    // // if color is valid, change color palette
    ui.text_edit_singleline(&mut value.edit_base_color);
    let data = value.edit_base_color.clone().to_ascii_uppercase();
    if Argb::from_str(&data).is_ok() {
        value.edit_base_color = data.clone();
        value.base_color = data;
    }

    // // this scope applies error colors to all elements inside
    ui.scope(|ui| {
        MaterialColors::new(value.base_color.clone(), value.dark_theme, 1.5).error_apply(ui);
        ui.button("Error button!")
    });

    let _ = ui.button("Regular button!");
    ui.label("Simple label");

    CollapsingHeader::new("Options")
        .default_open(value.options_open)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut value.dark_theme, "Dark Theme");
                ui.checkbox(&mut value.enabled, "Enabled");
            });
        });

    ui.add_enabled_ui(value.enabled, |ui| {
        Slider::new(&mut 50, 0..=100).trailing_fill(true).ui(ui);
    });

    ui.add_space(10.);

    ui.label("Whats your favorite color theme? personally mine is #AAE");
}
