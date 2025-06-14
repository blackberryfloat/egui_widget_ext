//! # Alert Demo Example
//!
//! This example demonstrates the usage of the `alert` widget and the `AlertLevel` enum
//! from the `egui_widget_ext` crate. It showcases various alert types (error, warning,
//! info, and success) with both short and long messages to test the alert widget's
//! appearance, text wrapping, and interaction handling.
//!
//! Run this example to see how different alert levels are rendered and how the widget
//! behaves with messages of varying lengths. Clicking an alert will dismiss it.
//!
//! To run this example:
//! ```sh
//! cargo run --example alerts
//! ```
//!
//! This file is intended for demonstration and manual testing purposes only.

use eframe::egui;

use egui_widget_ext::{Alert, AlertLevel, alert};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Alert Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(AlertsApp::new(cc)))),
    )
}

struct AppState {
    show_error: bool,
    show_error_long: bool,
    show_warning: bool,
    show_warning_long: bool,
    show_info: bool,
    show_info_long: bool,
    show_success: bool,
    show_success_long: bool,
}

struct AlertsApp {
    state: AppState,
}

impl Default for AlertsApp {
    fn default() -> Self {
        Self {
            state: AppState {
                show_error: true,
                show_error_long: true,
                show_warning: true,
                show_warning_long: true,
                show_info: true,
                show_info_long: true,
                show_success: true,
                show_success_long: true,
            },
        }
    }
}

impl AlertsApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AlertsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Alert::new("Welcome to the Alert Demo! This is not closeable.").with_level(AlertLevel::Info).can_close(false));
            if self.state.show_error {
                // Short error alert using the Alert struct directly
                use egui_widget_ext::Alert;
                ui.add(
                    Alert::new("This is an error alert!")
                        .with_level(AlertLevel::Error)
                        .corner_radius(8)
                        .inner_margin(12)
                )
                .clicked()
                .then(|| {
                    self.state.show_error = false;
                });
            }
            if self.state.show_error_long {
                // Very long error alert using the convenience function
                ui.add(alert(
                    AlertLevel::Error,
                    "ULTRA LONG ERROR ALERT: This is an extremely, almost absurdly long error alert message. Its purpose is to push the boundaries of the alert widget's text wrapping capabilities. The message continues with more and more text, describing in excruciating detail every possible scenario in which an error might occur, including but not limited to network failures, disk errors, memory leaks, unexpected panics, user misconfigurations, hardware malfunctions, cosmic rays, and even the heat death of the universe. If you can still read this message without horizontal scrolling or text overflowing outside the alert box, then the alert widget is truly robust. This message should wrap gracefully, maintaining readability and layout integrity no matter how much text is present. Keep adding more text to ensure that the alert box grows vertically and never horizontally, always respecting the boundaries of the parent UI container. Congratulations if you made it this far!",
                ))
                .clicked()
                .then(|| {
                    self.state.show_error_long = false;
                });
            }
            if self.state.show_warning {
                // Short warning alert using the Alert struct directly
                use egui_widget_ext::Alert;
                ui.add(
                    Alert::new("This is a warning alert!")
                        .with_level(AlertLevel::Warning)
                        .corner_radius(8)
                )
                .clicked()
                .then(|| {
                    self.state.show_warning = false;
                });
            }
            if self.state.show_warning_long {
                // Very long warning alert using the convenience function
                ui.add(alert(
                    AlertLevel::Warning,
                    "ULTRA LONG WARNING ALERT: This warning alert is so long that it might make you wonder if there is any end to it. The purpose is to ensure that the alert widget can handle even the most verbose and unnecessarily detailed warning messages, such as those that might be generated by an overzealous logging system or a particularly talkative developer. The text should wrap, never overflow, and always remain readable. If you see this message stretching off the edge of the window, something is wrong. Otherwise, everything is working as intended!",
                ))
                .clicked()
                .then(|| {
                    self.state.show_warning_long = false;
                });
            }
            if self.state.show_info {
                // Short info alert
                ui.add(alert(
                    AlertLevel::Info,
                    "This is an info alert!",
                ))
                .clicked()
                .then(|| {
                    self.state.show_info = false;
                });
            }
            if self.state.show_info_long {
                // Very long info alert
                ui.add(alert(
                    AlertLevel::Info,
                    "ULTRA LONG INFO ALERT: This informational alert contains an extraordinary amount of information, far more than any reasonable user would ever want to read in a single sitting. It is designed to test the absolute limits of the alert widget's ability to wrap and display text. If you can read this entire message without any part of it being cut off or overflowing its container, then the widget is performing admirably. Keep an eye out for any layout issues as the text continues to grow and grow, seemingly without end.",
                ))
                .clicked()
                .then(|| {
                    self.state.show_info_long = false;
                });
            }
            if self.state.show_success {
                // Short success alert
                ui.add(alert(
                    AlertLevel::Success,
                    "This is a success alert!",
                ))
                .clicked()
                .then(|| {
                    self.state.show_success = false;
                });
            }
            if self.state.show_success_long {
                // Very long success alert
                ui.add(alert(
                    AlertLevel::Success,
                    "ULTRA LONG SUCCESS ALERT: Congratulations! Not only have you succeeded, but you have done so in such a spectacular fashion that the success message itself cannot be contained in a single line. This message is intentionally verbose, overflowing with praise and accolades, to ensure that the alert widget can handle even the most exuberant celebrations of user achievement. The text should wrap, the alert box should expand vertically, and the user should be able to bask in the glory of their accomplishment without any UI issues.",
                ))
                .clicked()
                .then(|| {
                    self.state.show_success_long = false;
                });
            }
        });
    }
}
