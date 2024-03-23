use dotenv::dotenv;
use eframe::Frame;
use egui::{vec2, Context, Slider, Ui, Widget};
use material_colors::Argb;
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            base_color: env::var("BASE_COLOR").unwrap(),
            edit_base_color: env::var("BASE_COLOR").unwrap(),
            dark_theme: env::var("DARK_THEME").unwrap().parse().unwrap(),
            enabled: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // this single line applies the M3 theme, but the values can be hard coded as well
        material_egui::apply(self.base_color.clone(), self.dark_theme, 1.5, &ctx);
        //    material_egui::apply(String::from("F0F"), true, 1.5, &ctx);

        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }
}

fn update_fn(value: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut value.edit_base_color);

        let data = value.edit_base_color.clone().to_ascii_uppercase();
        if Argb::from_str(&data).is_ok() {
            value.edit_base_color = data.clone();
            value.base_color = data;
        }
    });
    // Slider::new( &mut value.base_tone, 10..=90, ).ui(ui);

    ui.checkbox(&mut value.dark_theme, "Dark Theme");
    // Palettes2::from_values(value.base_color.clone(), value.dark_theme)'

    let mut var = 50;

    ui.checkbox(&mut value.enabled, "Enabled");
    ui.add_enabled_ui(value.enabled, |ui| {
        Slider::new(&mut var, 0..=100).trailing_fill(true).ui(ui);
    });
}
