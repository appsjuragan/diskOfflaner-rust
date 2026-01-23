// src/gui/themes.rs
// Centralized theme configuration to eliminate duplication

use eframe::egui;

/// Apply dark mode visual settings
pub fn apply_dark_theme(ctx: &egui::Context) {
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

/// Apply light mode visual settings
pub fn apply_light_theme(ctx: &egui::Context) {
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
}
