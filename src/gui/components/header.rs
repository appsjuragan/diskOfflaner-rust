use eframe::egui;

pub fn show_header(ui: &mut egui::Ui, ctx: &egui::Context, is_loading: bool, on_refresh: impl FnOnce()) {
    ui.horizontal(|ui| {
        ui.heading("DiskOfflaner");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let is_dark = ctx.style().visuals.dark_mode;
            let text = if is_dark { "Light Mode" } else { "Dark Mode" };
            if ui.button(text).clicked() {
                if is_dark {
                    // Switch to Light Mode
                    let mut visuals = egui::Visuals::light();
                    let grey_95 = egui::Color32::from_gray(211);
                    let grey_input = egui::Color32::from_gray(225);

                    visuals.panel_fill = grey_95;
                    visuals.window_fill = grey_95;
                    visuals.widgets.noninteractive.bg_fill = grey_95;
                    visuals.extreme_bg_color = grey_input;

                    visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(190);
                    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(50));
                    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(170);
                    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(30));
                    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    visuals.widgets.active.bg_fill = egui::Color32::from_gray(150);
                    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
                    visuals.widgets.active.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    ctx.set_visuals(visuals);
                } else {
                    // Switch to Dark Mode
                    let mut visuals = egui::Visuals::dark();
                    visuals.extreme_bg_color = egui::Color32::from_gray(32);
                    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_gray(32);

                    visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(60);
                    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(220));
                    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(75);
                    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(240));
                    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    visuals.widgets.active.bg_fill = egui::Color32::from_gray(90);
                    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                    visuals.widgets.active.bg_stroke = egui::Stroke::new(0.0, egui::Color32::TRANSPARENT);

                    ctx.set_visuals(visuals);
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
