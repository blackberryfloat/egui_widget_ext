//! # Alert Manager Demo Example
//!
//! This example demonstrates the usage of the `AlertManager` widget from the
//! `egui_widget_ext` crate. It shows how to display alerts with different anchor
//! positions and in different UI regions. Use the buttons to trigger alerts in
//! various configurations and see how the alert manager handles stacking, scrolling,
//! and dismissal.

use eframe::egui;
use egui::{Align2, CentralPanel, Pos2, SidePanel, TopBottomPanel};
use egui_widget_ext::{AlertLevel, AlertManager};

struct AlertManagerApp {
    alerts: Vec<(AlertLevel, String)>,
    anchor: Align2,
    show_side_panel: bool,
    alert_width: Option<f32>, // Make width optional
}

impl Default for AlertManagerApp {
    fn default() -> Self {
        Self {
            alerts: vec![],
            anchor: Align2::CENTER_TOP,
            show_side_panel: false,
            alert_width: None, // Default: not set
        }
    }
}

impl AlertManagerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AlertManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Anchor: Top Center").clicked() {
                    self.anchor = Align2::CENTER_TOP;
                    self.alerts
                        .insert(0, (AlertLevel::Info, "Anchored to Top Center".into()));
                }
                if ui.button("Anchor: Bottom Center").clicked() {
                    self.anchor = Align2::CENTER_BOTTOM;
                    self.alerts
                        .insert(0, (AlertLevel::Warning, "Anchored to Bottom Center".into()));
                }
                if ui.button("Anchor: Top Left").clicked() {
                    self.anchor = Align2::LEFT_TOP;
                    self.alerts
                        .insert(0, (AlertLevel::Error, "Anchored to Top Left".into()));
                }
                if ui.button("Anchor: Bottom Right").clicked() {
                    self.anchor = Align2::RIGHT_BOTTOM;
                    self.alerts
                        .insert(0, (AlertLevel::Success, "Anchored to Bottom Right".into()));
                }
                if ui.button("Toggle Side Panel Alerts").clicked() {
                    self.show_side_panel = !self.show_side_panel;
                    if self.show_side_panel {
                        self.alerts
                            .insert(0, (AlertLevel::Info, "Side panel alerts enabled!".into()));
                    } else {
                        self.alerts.insert(
                            0,
                            (AlertLevel::Warning, "Side panel alerts disabled!".into()),
                        );
                    }
                }
                if ui.button("Add Many Alerts").clicked() {
                    for i in 0..10 {
                        self.alerts.insert(
                            0,
                            (
                                AlertLevel::Info,
                                format!("Bulk alert #{i} (scroll to see more)"),
                            ),
                        );
                    }
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Alert Width:");
                // Use a temporary variable for editing
                let mut width = self.alert_width.unwrap_or(300.0);
                let response = ui.add(egui::DragValue::new(&mut width).speed(1.0));
                if response.changed() {
                    self.alert_width = Some(width);
                }
                if ui.button("Set Alert Width").clicked() {
                    self.alert_width = Some(width);
                    self.alerts.insert(
                        0,
                        (AlertLevel::Info, format!("Alert width set to {:.0}", width)),
                    );
                }
                if ui.button("Clear Width").clicked() {
                    self.alert_width = None;
                    self.alerts.insert(
                        0,
                        (
                            AlertLevel::Info,
                            "Alert width cleared (using default)".into(),
                        ),
                    );
                }
                if let Some(w) = self.alert_width {
                    ui.label(format!("Current: {:.0}", w));
                } else {
                    ui.label("Current: default");
                }
            });
        });

        if self.show_side_panel {
            SidePanel::left("side_panel").show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Side Panel");
                if ui.button("Add Side Alert").clicked() {
                    self.alerts
                        .push((AlertLevel::Warning, "Alert in side panel!".into()));
                }
                let mut alert_manager = AlertManager::new(&mut self.alerts, "side_panel")
                    .anchor(Align2::LEFT_TOP)
                    .max_height(200.0);
                if let Some(width) = self.alert_width {
                    alert_manager = alert_manager.width(width);
                }
                ui.add(alert_manager);
            });
        }

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            let pos: Option<Pos2> = ctx.input(|i| i.pointer.hover_pos());
            if let Some(pos) = pos {
                ui.label(format!(
                    "Cursor position: x = {:.1}, y = {:.1}",
                    pos.x, pos.y
                ));
            } else {
                ui.label("Cursor position: <not hovering>");
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alert Manager Demo");
            ui.label("Use the buttons above to trigger alerts in different positions.");
            if !self.show_side_panel {
                let mut alert_manager =
                    AlertManager::new(&mut self.alerts, "main").anchor(self.anchor);
                if let Some(width) = self.alert_width {
                    alert_manager = alert_manager.width(width);
                }
                ui.add(alert_manager);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Alert Manager Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(AlertManagerApp::new(cc)))),
    )
}
