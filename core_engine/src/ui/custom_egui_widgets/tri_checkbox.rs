use bevy_egui::egui::{Align2, Response, Sense, TextStyle, Widget};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TriState {
    Unchecked,
    Indeterminate,
    Checked,
}

pub struct TriCheckbox<'a> {
    state: &'a mut TriState,
}
impl<'a> TriCheckbox<'a> {
    pub fn new(state: &'a mut TriState) -> Self {
        Self {
            state,
        }
    }
}
impl Widget for TriCheckbox<'_> {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> Response {
        let desired_size = ui.spacing().interact_size;
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            *self.state = match *self.state {
                TriState::Unchecked => TriState::Indeterminate,
                TriState::Indeterminate => TriState::Checked,
                TriState::Checked => TriState::Unchecked,
            };
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let painter = ui.painter_at(rect);

            // Draw box background and border
            painter.rect(rect, 0.0, visuals.bg_fill, visuals.bg_stroke);

            // Draw symbol for current state
            let center = rect.center();
            let symbol = match *self.state {
                TriState::Unchecked => Some("-"),
                TriState::Indeterminate => Some("?"),
                TriState::Checked => Some("+"),
            };

            if let Some(sym) = symbol {
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    sym,
                    TextStyle::Button.resolve(ui.style()),
                    visuals.text_color(),
                );
            }
        }

        response
    }
}

pub trait TriCheckboxExt {
    fn tri_checkbox(&mut self, state: &mut TriState) -> Response;
}

impl TriCheckboxExt for bevy_egui::egui::Ui {
    fn tri_checkbox(&mut self, state: &mut TriState) -> Response {
        TriCheckbox::new(state).ui(self)
    }
}
