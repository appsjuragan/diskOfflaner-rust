use crate::gui::themes;
use eframe::egui;

pub fn show_header(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    is_loading: bool,
    on_refresh: impl FnOnce(),
) {
    ui.horizontal(|ui| {
        ui.heading("DiskOfflaner");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let is_dark = ctx.style().visuals.dark_mode;
            let text = if is_dark { "Light Mode" } else { "Dark Mode" };
            if ui.button(text).clicked() {
                if is_dark {
                    themes::apply_light_theme(ctx);
                } else {
                    themes::apply_dark_theme(ctx);
                }
            }

            if is_loading {
                ui.add_enabled(false, egui::Button::new("⟳ Refreshing..."));
                ui.spinner();
            } else if ui.button("⟳ Refresh").clicked() {
                on_refresh();
            }
        });
    });
}
