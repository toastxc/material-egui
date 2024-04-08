pub mod widgets;
pub use widgets::button::Button;

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{epaint, Color32, Context, Stroke, Style, Ui, Visuals};
use material_colors::Argb;
use std::str::FromStr;

#[derive(Clone, Default, Debug)]
pub struct MaterialColors {
    pub dark: bool,
    pub base_color: String,
    pub zoom: f32,
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

impl MaterialColors {
    pub fn rebuild(&mut self) -> Self {
        *self = Self::new(self.base_color.clone(), self.dark, self.zoom).clone();
        self.clone()
    }
    pub fn new(base_color: String, dark: bool, zoom: f32) -> Self {
        let data = material_colors::theme_from_source_color(
            Argb::from_str(&base_color).unwrap(),
            Default::default(),
        );

        let scheme = match dark {
            true => data.schemes.dark,
            false => data.schemes.light,
        };

        Self {
            base_color,
            dark,
            zoom,

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
    /// exports the default M3 theme
    /// this function applies M3 themes to egui assets, specific implementation is subject to change
    /// CANNOT apply zoom
    pub fn export(&self) -> Style {
        let p = self;

        let visuals = Visuals {
            // override_text_color: Some(p.primary),
            collapsing_header_frame: true,
            override_text_color: None,
            hyperlink_color: p.on_primary,
            // background
            faint_bg_color: p.surface_container,
            extreme_bg_color: p.surface_variant,
            code_bg_color: p.surface_dim,

            window_fill: p.surface_container_highest,
            panel_fill: p.surface,

            warn_fg_color: p.error_container,
            error_fg_color: p.error,

            // window_stroke: Stroke {
            //     color: p.secondary,
            //     width: 10.5,
            // },

            window_highlight_topmost: false,
            // todo
            widgets: widgets(p.clone()),
            window_shadow: epaint::Shadow {
                color: p.shadow,
                ..Default::default()
            },

            popup_shadow: epaint::Shadow {
                color: p.shadow,
                ..Default::default()
            },

            // todo selection represents a kind of accent color for widgets outside of the widgets config
            // this is the subject of an issue https://github.com/emilk/egui/issues/4227
            selection: Selection {
                bg_fill: p.secondary,
                stroke: Stroke {
                    width: 1.5,
                    color: p.on_secondary,
                },
            },

            ..Default::default()
        };

        Style {
            visuals,

            ..Default::default()
        }
    }
    /// applies generated theming values to ctx
    /// this also applies zoom value: must be stored persistently
    pub fn apply_zoom(&mut self, ctx: &Context, first_run: &mut bool) {
        if *first_run {
            ctx.set_zoom_factor(self.zoom);
            *first_run = false;
        }
        ctx.set_style(self.export());
    }

    /// applies generated theming values to ctx
    pub fn apply(&mut self, ctx: &Context) {
        ctx.set_style(self.export());
    }
    /// applies generated values to Ui
    /// CANNOT apply zoom
    pub fn apply_ui(&self, ui: &mut Ui) {
        ui.set_style(self.export())
    }

    pub fn error_apply(&self, ui: &mut Ui) {
        let widgets = &mut ui.style_mut().visuals.widgets;
        // 'error buttons' are not documented in M3, so its unclear which arrangement is better
        let active = (self.error, self.on_error);
        let rest = (self.error_container, self.on_error_container);
        // widgets
        widget_maker_mut(&mut widgets.hovered, rest.0, rest.1);
        widget_maker_mut(&mut widgets.inactive, rest.0, rest.1);
        widget_maker_mut(&mut widgets.open, active.0, active.1);
        widget_maker_mut(&mut widgets.active, active.0, active.1);
    }
}
fn widgets(p: MaterialColors) -> Widgets {
    let mut old = match p.dark {
        true => Visuals::dark(),
        false => Visuals::light(),
    }
    .widgets;

    //   colors.visuals.widgets.noninteractive.bg_stroke.color = Color32::RED;
    widget_maker_mut(&mut old.noninteractive, p.surface, p.on_surface);

    old.noninteractive.bg_stroke.color = p.surface_variant;
    old.noninteractive.bg_stroke.width = 4.;

    widget_maker_mut(
        &mut old.inactive,
        p.primary_container,
        p.on_primary_container,
    );
    widget_maker_mut(
        &mut old.hovered,
        p.tertiary_container,
        p.on_tertiary_container,
    );
    widget_maker_mut(&mut old.active, p.tertiary, p.on_tertiary);
    widget_maker_mut(&mut old.open, p.primary_container, p.on_primary_container);
    old
}
fn widget_maker_mut(old: &mut WidgetVisuals, fill: Color32, text: Color32) {
    old.bg_fill = fill;
    old.weak_bg_fill = fill;
    old.fg_stroke.color = text;
    old.bg_stroke.color = fill;
    old.expansion = 0.;
}
