//! This is where we'll do all the GUI stuff.
//!
use egui::*;
use egui_extras::RetainedImage;

use crate::consts::PREVIEW_IMAGE_BYTES;
// use poll_promise::Promise; // We're going to use this later when we're downloading images...

/// This is the main struct that Egui's required trait etc are implemented on,
/// it's also the place we can stash data!
/// egui apps have a context, but it's significantly more ergonomic to stash things _here_.
pub struct MruApp {
    /// This is the App's RHS preview image.
    preview: RetainedImage,
    /// False when the 'are you sure you want to quit' pop is active.
    allowed_to_close: bool,
    #[cfg(not(debug_assertions))] // Let's assume developers can quit when they want to.
    show_confirmation_dialog: bool,
}

impl Default for MruApp {
    fn default() -> Self {
        let preview =
            RetainedImage::from_image_bytes(String::from("PreviewImg"), PREVIEW_IMAGE_BYTES)
                .expect("As this is loaded from a const, it should never fail.");
        Self {
            preview,
            allowed_to_close: false,
                #[cfg(not(debug_assertions))] // Let's assume developers can quit when they want to.

            show_confirmation_dialog: false,
        }
    }
}
impl MruApp {
    #[cfg(not(debug_assertions))] // Let's assume developers can quit when they want to.
    fn on_close(&self) {
        if self.show_confirmation_dialog {
            egui::Window::new("Are you sure you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

impl eframe::App for MruApp {
    #[cfg(not(debug_assertions))] // Let's assume developers can quit when they want to.
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Are you sure you want to quit?
        //TODO: control where on screen this spawns, make that spawner modular so we can use it elsewhere.
        #[cfg(not(debug_assertions))] // Let's assume developers can quit when they want to.
        self.on_close();

        //EFRAME has some cool styling options etc
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            rounding: 10.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            ..Default::default()
        };
        //TOP
        egui::TopBottomPanel::top("TOP_PANEL")
            .frame(panel_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();
                let title_bar_height = 24.0;
                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + title_bar_height;
                    rect
                };
                title_bar_ui(ui, frame, title_bar_rect, "MRU-UI");
            });

        //RHS
        egui::SidePanel::right("LHS_PANEL")
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.horizontal(|ui| {
                        self.preview.show(ui);
                    });
                });
            });
        //LHS
        egui::SidePanel::left("RHS_PANEL")
            .frame(panel_frame)
            .show(ctx, |ui| {
                // A gallery, could be cool here?
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    ui.label("First row, first column");
                    ui.label("First row, second column");
                    ui.end_row();

                    ui.label("Second row, first column");
                    ui.label("Second row, second column");
                    ui.label("Second row, third column");
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Same");
                        ui.label("cell");
                    });
                    ui.label("Third row, second column");
                    ui.end_row();
                });
            });

        //CENTER
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Center Panel");
        });
    }
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}
