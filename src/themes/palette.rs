use egui::Color32;
use material_colors::Argb;
use std::str::FromStr;
#[derive(Clone, Default)]
pub struct Palettes2 {
    pub dark: bool,
    pub base_color: String,
    pub primary: Color32,
    pub on_primary: Color32,
    pub primary_container: Color32,
    pub on_primary_container: Color32,
    pub inverse_primary: Color32,
    pub primary_fixed: Color32,
    pub primary_fixed_dim: Color32,
    pub on_primary_fixed: Color32,
    pub on_primary_fixed_variant: Color32,
    pub secondary: Color32,
    pub on_secondary: Color32,
    pub secondary_container: Color32,
    pub on_secondary_container: Color32,
    pub secondary_fixed: Color32,
    pub secondary_fixed_dim: Color32,
    pub on_secondary_fixed: Color32,
    pub on_secondary_fixed_variant: Color32,
    pub tertiary: Color32,
    pub on_tertiary: Color32,
    pub tertiary_container: Color32,
    pub on_tertiary_container: Color32,
    pub tertiary_fixed: Color32,
    pub tertiary_fixed_dim: Color32,
    pub on_tertiary_fixed: Color32,
    pub on_tertiary_fixed_variant: Color32,
    pub error: Color32,
    pub on_error: Color32,
    pub error_container: Color32,
    pub on_error_container: Color32,
    pub surface_dim: Color32,
    pub surface: Color32,
    pub surface_bright: Color32,
    pub surface_container_lowest: Color32,
    pub surface_container_low: Color32,
    pub surface_container: Color32,
    pub surface_container_high: Color32,
    pub surface_container_highest: Color32,
    pub on_surface: Color32,
    pub on_surface_variant: Color32,
    pub outline: Color32,
    pub outline_variant: Color32,
    pub inverse_surface: Color32,
    pub inverse_on_surface: Color32,
    pub surface_variant: Color32,
    pub background: Color32,
    pub on_background: Color32,
    pub shadow: Color32,
    pub scrim: Color32,
}

fn c(i: Argb) -> Color32 {
    Color32::from_rgba_premultiplied(i.red, i.green, i.blue, i.alpha)
}

impl Palettes2 {
    pub fn from_values(base_color: impl Into<String>, dark: bool) -> Self {
        let base_color = base_color.into();
        let scheme = material_colors::theme_from_source_color(
            Argb::from_str(&base_color).unwrap(),
            Default::default(),
        )
        .schemes;
        let scheme = match dark {
            true => scheme.dark,
            false => scheme.light,
        };

        Self {
            base_color,
            dark,
            // primary
            primary: c(scheme.primary),
            on_primary: c(scheme.on_primary),
            primary_container: c(scheme.primary_container),
            on_primary_container: c(scheme.on_primary_container),
            inverse_primary: c(scheme.inverse_primary),
            primary_fixed: c(scheme.primary_fixed),
            primary_fixed_dim: c(scheme.primary_fixed_dim),
            on_primary_fixed: c(scheme.on_primary_fixed),
            on_primary_fixed_variant: c(scheme.on_primary_fixed_variant),
            // secondary
            secondary: c(scheme.secondary),
            on_secondary: c(scheme.on_secondary),
            secondary_container: c(scheme.secondary_container),
            on_secondary_container: c(scheme.on_secondary_container),
            secondary_fixed: c(scheme.secondary_fixed),
            secondary_fixed_dim: c(scheme.secondary_fixed_dim),
            on_secondary_fixed: c(scheme.on_secondary_fixed),
            on_secondary_fixed_variant: c(scheme.on_secondary_fixed_variant),
            // tertiary
            tertiary: c(scheme.tertiary),
            on_tertiary: c(scheme.on_tertiary),
            tertiary_container: c(scheme.tertiary_container),
            on_tertiary_container: c(scheme.on_tertiary_container),
            tertiary_fixed: c(scheme.tertiary_fixed),
            tertiary_fixed_dim: c(scheme.tertiary_fixed_dim),
            on_tertiary_fixed: c(scheme.on_tertiary_fixed),
            on_tertiary_fixed_variant: c(scheme.on_tertiary_fixed_variant),
            // error
            error: c(scheme.error),
            on_error: c(scheme.on_error),
            error_container: c(scheme.error_container),
            on_error_container: c(scheme.on_error_container),
            // surface
            surface_dim: c(scheme.surface_dim),
            surface: c(scheme.surface),
            surface_bright: c(scheme.surface_bright),
            surface_container_lowest: c(scheme.surface_container_lowest),
            surface_container_low: c(scheme.surface_container_low),
            surface_container: c(scheme.surface_container),
            surface_container_high: c(scheme.surface_container_high),
            surface_container_highest: c(scheme.surface_container_highest),
            on_surface: c(scheme.on_surface),
            on_surface_variant: c(scheme.on_surface_variant),
            // outline
            outline: c(scheme.outline),
            outline_variant: c(scheme.outline_variant),
            // inverse
            inverse_surface: c(scheme.inverse_surface),
            inverse_on_surface: c(scheme.inverse_on_surface),
            surface_variant: c(scheme.surface_variant),
            background: c(scheme.background),
            on_background: c(scheme.on_background),
            shadow: c(scheme.shadow),
            scrim: c(scheme.scrim),
        }
    }
}