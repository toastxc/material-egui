use egui::{
    epaint,
    style::{Selection, WidgetVisuals, Widgets},
    Color32, Context, Style, Visuals,
};

use crate::themes::palette::Palettes2;

pub fn apply(base_color: String, dark: bool, zoom: f32, ctx: &Context) {
    let p = Palettes2::from_values(base_color, dark);

    let visuals = Visuals {
        override_text_color: Some(p.primary),
        hyperlink_color: p.on_primary,
        // background
        faint_bg_color: p.surface_container,
        extreme_bg_color: p.surface_variant,
        code_bg_color: p.surface_dim,

        window_fill: p.surface_container_highest,
        panel_fill: p.surface,

        warn_fg_color: p.error_container,
        error_fg_color: p.error,

        window_stroke: egui::Stroke {
            color: p.secondary,
            ..Default::default()
        },
        window_highlight_topmost: false,
        widgets: widgets(p.clone()),
        window_shadow: epaint::Shadow {
            color: p.shadow,

            ..Default::default()
        },

        popup_shadow: epaint::Shadow {
            color: p.shadow,
            ..Default::default()
        },

        selection: Selection {
            bg_fill: p.tertiary,
            stroke: Default::default(),
        },
        ..Default::default()
    };

    let default = Style {
        visuals,
        ..Default::default()
    };

    ctx.set_style(default);
    ctx.set_zoom_factor(zoom);
}
fn widgets(p: Palettes2) -> Widgets {
    let old = Visuals::dark().widgets;

    Widgets {
        noninteractive: widget_maker(old.noninteractive, p.surface, p.surface_container),
        inactive: widget_maker(old.inactive, p.primary_container, p.on_primary_container),
        hovered: widget_maker(old.hovered, p.tertiary_container, p.on_tertiary_container),
        active: widget_maker(old.active, p.inverse_primary, p.on_primary_container),
        open: widget_maker(old.open, p.primary_container, p.on_primary_container),
    }
}

fn widget_maker(old: WidgetVisuals, c1: Color32, c2: Color32) -> WidgetVisuals {
    let mut old = old;
    old.bg_fill = c1;
    old.weak_bg_fill = c1;
    old.fg_stroke.color = c2;
    old.bg_stroke.color = c2;
    old
}
