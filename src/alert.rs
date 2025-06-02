//! # Alert Widget Module
//!
//! This module provides a customizable alert box widget for use with the `egui` GUI library.
//! The alert box displays a message with a severity level (success, info, warning, error) and
//! includes a close ("✕") button. The appearance of the alert can be customized via margins and corner radius.
//!
//! ## Example
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_widget_ext::{alert, AlertLevel};
//! ui.add(alert(AlertLevel::Warning, "This is a warning!")).clicked().then(|| {
//!     println!("Alert clicked!");
//! });
//! # });
//! ```
//!
//! ## Components
//! - [`AlertLevel`]: Enum representing the severity of the alert.
//! - [`Alert`]: Struct for configuring and displaying the alert widget.
//! - [`alert`]: Convenience function for creating an alert widget.

use egui::{Button, Color32, CornerRadius, Frame, Label, Margin, RichText, Stroke, Ui, Widget};

/// Represents the severity level of an alert. Determines the background color and semantic meaning
/// of the alert box.
///
/// - `Success`: Indicates a successful operation or state (green).
/// - `Info`: Indicates informational messages that are not critical (blue).
/// - `Warning`: Indicates a warning that may require attention but is not critical (yellow).
/// - `Error`: Indicates an error or critical issue that needs immediate attention (red).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    /// Indicates a successful operation or state.
    Success,
    /// Indicates informational messages that are not critical.
    Info,
    /// Indicates a warning that may require attention but is not critical.
    Warning,
    /// Indicates an error or critical issue that needs immediate attention.
    Error,
}

/// A customizable alert box widget for egui.
///
/// The `Alert` struct allows you to configure the appearance and message of the alert box.
/// It supports setting the background color (via [`AlertLevel`]), the message, inner and outer margins,
/// and the corner radius. The alert box always includes a close ("✕") button.
///
/// Use the [`alert`] function for a convenient way to create an alert with a given level and message.
#[derive(Debug, Clone)]
pub struct Alert {
    /// The background color of the alert box.
    color: Color32,
    /// The message displayed in the alert box.
    message: String,
    /// Padding inside the alert box.
    inner_margin: i8,
    /// Margin outside the alert box.
    outer_margin: i8,
    /// Corner radius of the alert box.
    corner_radius: u8,
    /// Whether to show the close ("✕") button.
    can_close: bool,
}

impl Default for Alert {
    /// Creates a default alert with a generic error color and message.
    fn default() -> Self {
        Alert {
            color: Color32::from_rgb(255, 200, 200),
            message: "No message provided".to_string(),
            inner_margin: 10,
            outer_margin: 10,
            corner_radius: 4,
            can_close: true, // Show close button by default
        }
    }
}

impl Alert {
    /// Create a new alert with the given message and default info color.
    pub fn new(message: &str) -> Self {
        let color = Self::level_to_color(AlertLevel::Info);
        Self {
            color,
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// Set the alert's severity level, which determines its background color.
    pub fn with_level(mut self, level: AlertLevel) -> Self {
        self.color = Self::level_to_color(level);
        self
    }

    /// Set the inner margin (padding) of the alert box.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = margin;
        self
    }

    /// Set the outer margin of the alert box.
    pub fn outer_margin(mut self, margin: i8) -> Self {
        self.outer_margin = margin;
        self
    }

    /// Set the corner radius of the alert box.
    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set whether the close ("✕") button is shown.
    pub fn can_close(mut self, closeable: bool) -> Self {
        self.can_close = closeable;
        self
    }

    /// Map an [`AlertLevel`] to its corresponding background color.
    fn level_to_color(level: AlertLevel) -> Color32 {
        match level {
            AlertLevel::Success => Color32::LIGHT_GREEN,
            AlertLevel::Info => Color32::LIGHT_BLUE,
            AlertLevel::Warning => Color32::LIGHT_YELLOW,
            AlertLevel::Error => Color32::LIGHT_RED,
        }
    }
}

impl Widget for Alert {
    /// Render the alert widget in the given egui UI context.
    ///
    /// The alert is displayed as a colored frame with the message and an optional close button.
    /// The returned [`egui::Response`] covers both the label and the close button (if present).
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let frame = Frame::default()
            .fill(self.color)
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
            .corner_radius(CornerRadius::same(self.corner_radius))
            .inner_margin(Margin::same(self.inner_margin))
            .outer_margin(Margin::same(self.outer_margin));

        frame
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let label_resp = ui
                        .add(Label::new(RichText::new(&self.message).color(Color32::BLACK)).wrap());
                    let response = if self.can_close {
                        let close_resp = ui
                            .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add(
                                    Button::new(
                                        RichText::new("X").color(Color32::DARK_RED).strong(),
                                    )
                                    .frame(false),
                                )
                            })
                            .inner;
                        label_resp | close_resp
                    } else {
                        ui.add_space(ui.available_width());
                        label_resp
                    };
                    response
                })
                .inner
            })
            .inner
    }
}

/// Convenience function to create an alert widget with a given level and message.
///
/// # Parameters
/// - `level`: The [`AlertLevel`] of the alert, which determines the background color.
/// - `message`: The message to display inside the alert box.
///
/// # Returns
/// Returns an [`egui::Widget`] closure. When invoked, it returns an [`egui::Response`] for the alert box.
///
/// # Example
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_widget_ext::{alert, AlertLevel};
/// ui.add(alert(AlertLevel::Warning, "This is a warning!")).clicked().then(|| {
///     println!("Alert clicked!");
/// });
/// # });
/// ```
pub fn alert(level: AlertLevel, message: &str) -> impl Widget + '_ {
    move |ui: &mut Ui| ui.add(Alert::new(message).with_level(level))
}
