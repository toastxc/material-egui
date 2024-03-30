use egui::{Align2, Color32, FontId, Id, LayerId, Order, Response, Stroke, Ui};
use crate::MaterialColors;
#[derive(Clone, Default, Debug)]
pub struct Button {
    style: MaterialColors
}
impl Button {
    pub fn new(style: &MaterialColors) -> Self {
        Self {
            style: style.clone(),
        }
    }
    //
    pub fn elevated(&self, ui: &mut Ui, text: impl Into<String>) -> Response {
        Self::m3_button(
            ui,
            text,
            self.style.surface_container_low,
            self.style.primary,
            Self::stroke_null(),
        )
    }
    pub fn filled(&self, ui: &mut Ui, text: impl Into<String>) -> Response {
        Self::m3_button(
            ui,
            text,
            self.style.primary,
            self.style.on_primary,
            Self::stroke_null(),
        )
    }
    pub fn filled_tonal(&self, ui: &mut Ui, text: impl Into<String>) -> Response {
        Self::m3_button(
            ui,
            text,
            self.style.primary_container,
            self.style.on_primary_container,
            Self::stroke_null(),
        )
    }
    pub fn outlined(&self, ui: &mut Ui, text: impl Into<String>) -> Response {
        Self::m3_button(
            ui,
            text,
            self.style.surface,
            self.style.primary,
            Stroke {
                width: 1.5,
                color: self.style.outline,
            },
        )
    }

    fn m3_button(
        ui: &mut egui::Ui,
        text: impl Into<String>,
        container_style: Color32,
        text_style: Color32,
        stroke: Stroke,
    ) -> egui::Response {
        let text = text.into().replace('\n', "");

        // initial rectangle
        let scale = 1.5;

        let desired_size = ui.spacing().interact_size.y * egui::vec2(scale * 2., scale);
        let (mut rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // styling
        let mut container_style = container_style;
        if response.hovered() {
            container_style = Self::delta(container_style, 10);
        };

        let mut width = 0.;
        let mut height = 0.;
        let mut position = rect.left_center();
        position.x += 10.;

        ui.with_layer_id(
            LayerId {
                order: Order::Foreground,
                id: Id::new("foreground"),
            },
            |ui| {
                let ui = ui.painter().text(
                    position,
                    Align2::LEFT_CENTER,
                    text,
                    FontId::proportional(rect.width() / 4.5),
                    text_style,
                );
                width = ui.width();
                height = ui.height();
            },
        );

        // box
        let radius = 0.5 * rect.height();
        rect.set_width(width + 20.);

        ui.with_layer_id(LayerId::background(), |ui| {
            ui.painter().rect(rect, radius, container_style, stroke);
        });

        response
    }


    fn stroke_null() -> Stroke {
        Stroke {
            width: 0.0,
            color: Default::default(),
        }
    }

    // makes the color lighter or darker depending on the colors brightness
    fn delta(i: Color32, change: u8) -> Color32 {
        let f = change;
        let (mut r, mut g, mut b, a) = (i.r(), i.g(), i.b(), i.a());
        if (r as u32 + g as u32 + b as u32) > 384 {
            r -= f;
            g -= f;
            b -= f;
        } else {
            r += f;
            g += f;
            b += f;
        }
        Color32::from_rgba_premultiplied(r, g, b, a)
    }
}