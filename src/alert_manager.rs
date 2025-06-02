//! # Alert Manager Widget Module
//!
//! This module provides an `AlertManager` widget for managing and displaying a stack of alert messages
//! in an egui application. The `AlertManager` is designed to help you present multiple alerts with
//! consistent styling, positioning, and dismissal behavior.
//!
//! ## Usage
//!
//! The `AlertManager` takes a mutable reference to a `Vec<(AlertLevel, String)>` representing the current
//! alerts to display. It provides builder-style methods to configure margins, corner radius, width, anchor
//! alignment, custom position, anchor offset, and maximum height for the alert area. Each alert is rendered
//! using the `Alert` widget, and closed alerts are automatically removed from the vector.
//!
//! **Recommended:**  
//! Place the `AlertManager` at the root level of your egui widget tree (such as inside your `CentralPanel`,
//! or as a top-level overlay) to ensure that alerts are positioned and layered correctly above
//! your main content. However, you can technically use it anywhere in your widget hierarchy if you
//! want to scope alerts to a specific region or panel.
//!
//! ## Example
//! ```rust
//! # use egui_widget_ext::{AlertManager, AlertLevel};
//! # use egui::{CentralPanel, Context};
//! # fn ui_example(ctx: &Context, alerts: &mut Vec<(AlertLevel, String)>) {
//! CentralPanel::default().show(ctx, |ui| {
//!     AlertManager::new(alerts)
//!         .corner_radius(8)
//!         .width(400.0)
//!         .anchor(egui::Align2::CENTER_TOP)
//!         .max_height(300.0)
//!         .ui(ui);
//! });
//! # }
//! ```
//!
//! ## Features
//! - Shared styling for all alerts (margin, corner radius, width, etc.)
//! - Configurable anchor alignment and custom position
//! - Optional anchor offset for fine-tuned placement
//! - Automatic removal of closed alerts from the vector
//! - Scrollable area if alerts exceed the maximum height
//!
//! ## Note
//! The alert manager is intended for use with the `Alert` widget and expects each alert to be a tuple of
//! `(AlertLevel, String)`. You can push new alerts to the vector at any time, and they will be displayed
//! until dismissed by the user.

use egui::{Align2, Id, Order, ScrollArea, Ui, UiBuilder, Vec2, Widget, WidgetWithState};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{Alert, AlertLevel};

pub struct AlertManagerState {
    pub alerts_rect: egui::Rect,
}

/// Manages and displays a list of alerts with shared styling and positioning.
///
/// See the [module-level documentation](self) for usage and configuration details.
#[derive(Debug)]
pub struct AlertManager<'a> {
    /// Unique key for the alert manager instance (used for state management).
    pub unique_key: String,
    /// List of alerts as (level, message) tuples.
    pub alerts: &'a mut Vec<(AlertLevel, String)>,
    /// Default inner margin for alerts.
    pub inner_margin: i8,
    /// Default outer margin for alerts.
    pub outer_margin: i8,
    /// Default corner radius for alerts.
    pub corner_radius: u8,
    /// Default width for the alert area (optional).
    pub width: Option<f32>,
    /// Whether alerts can be closed.
    pub can_close: bool,
    /// Anchor position for the alert stack.
    pub anchor: Align2,
    /// Optional offset from the anchor position.
    pub anchor_offset: Option<Vec2>,
    /// Optional maximum height for the alert area (enables scrolling if exceeded).
    pub max_height: Option<f32>,
}

impl<'a> AlertManager<'a> {
    /// Create a new alert manager with a reference to a list of alerts.
    pub fn new(alerts: &'a mut Vec<(AlertLevel, String)>, unique_key: &str) -> Self {
        Self {
            unique_key: format!("alert_manager_{}", unique_key),
            alerts,
            inner_margin: 10,
            outer_margin: 1,
            corner_radius: 4,
            width: None,
            can_close: true,
            anchor: Align2::CENTER_TOP, // Default to top center
            anchor_offset: None,
            max_height: None,
        }
    }

    /// Set the inner margin for all alerts.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = margin;
        self
    }

    /// Set the outer margin for all alerts.
    pub fn outer_margin(mut self, margin: i8) -> Self {
        self.outer_margin = margin;
        self
    }

    /// Set the corner radius for all alerts.
    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the width for the alert area.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set whether alerts can be closed.
    pub fn can_close(mut self, can_close: bool) -> Self {
        self.can_close = can_close;
        self
    }

    /// Set the anchor position for the alert stack.
    pub fn anchor(mut self, anchor: Align2) -> Self {
        let is_valid = matches!(
            anchor,
            Align2::LEFT_TOP
                | Align2::CENTER_TOP
                | Align2::RIGHT_TOP
                | Align2::LEFT_BOTTOM
                | Align2::CENTER_BOTTOM
                | Align2::RIGHT_BOTTOM
        );
        assert!(
            is_valid,
            "Invalid anchor position: {:?}. We only support top or bottom anchors.",
            anchor
        );
        self.anchor = anchor;
        self
    }

    /// Set an offset from the anchor position.
    pub fn anchor_offset(mut self, offset: Vec2) -> Self {
        self.anchor_offset = Some(offset);
        self
    }

    /// Set the maximum height for the alert area (enables scrolling if exceeded).
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }
}

impl<'a> WidgetWithState for AlertManager<'a> {
    type State = AlertManagerState;
}

impl<'a> Widget for AlertManager<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let parent_area = ui.max_rect();
        let mut to_remove = Vec::new();

        // Generate a unique area for unique alert manager data
        // TODO: this may be a memory leak
        let mut hasher = DefaultHasher::new();
        self.alerts.hash(&mut hasher);
        let alerts_hash = hasher.finish();
        let id: Id = Id::new(format!(
            "{}_{}_{}_{}",
            self.unique_key,
            self.width.unwrap_or(-1.0),
            self.max_height.unwrap_or(-1.0),
            alerts_hash
        ));

        let is_bottom = self.anchor == Align2::LEFT_BOTTOM
            || self.anchor == Align2::CENTER_BOTTOM
            || self.anchor == Align2::RIGHT_BOTTOM;
        let max_height = self
            .max_height
            .unwrap_or(parent_area.height())
            .min(parent_area.height());
        let max_width = self
            .width
            .unwrap_or(parent_area.width())
            .min(parent_area.width());

        let resp = egui::Area::new(id)
            .order(Order::Foreground)
            .anchor(self.anchor, self.anchor_offset.unwrap_or(Vec2::ZERO))
            .constrain_to(parent_area)
            .default_size(Vec2::new(max_width, max_height))
            .show(ui.ctx(), |ui| {
                if !ui.is_enabled() && !ui.is_visible() {
                    for (level, message) in self.alerts.iter() {
                        let mut alert = Alert::new(message)
                            .with_level(*level)
                            .inner_margin(self.inner_margin)
                            .outer_margin(self.outer_margin)
                            .corner_radius(self.corner_radius)
                            .can_close(self.can_close);
                        if self.width.is_some() {
                            alert = alert.width(self.width.unwrap());
                        }
                        ui.add(alert);
                    }
                } else {
                    // Normal pass: use ScrollArea
                    let scroll_resp = ScrollArea::both()
                        .stick_to_bottom(is_bottom)
                        .max_height(max_height)
                        .max_width(max_width)
                        .show(ui, |ui| {
                            let alert_iter: Box<
                                dyn Iterator<Item = (usize, &(AlertLevel, String))>,
                            > = if is_bottom {
                                Box::new(self.alerts.iter().enumerate().rev())
                            } else {
                                Box::new(self.alerts.iter().enumerate())
                            };

                            for (idx, (level, message)) in alert_iter {
                                let mut alert = Alert::new(message)
                                    .with_level(*level)
                                    .inner_margin(self.inner_margin)
                                    .outer_margin(self.outer_margin)
                                    .corner_radius(self.corner_radius)
                                    .can_close(self.can_close);
                                if self.width.is_some() {
                                    alert = alert.width(self.width.unwrap());
                                }
                                let resp = ui.add(alert);
                                if self.can_close && resp.clicked() {
                                    to_remove.push(idx);
                                }
                            }

                            for idx in to_remove.into_iter().rev() {
                                self.alerts.remove(idx);
                            }
                        });
                    scroll_resp.inner
                }
            })
            .response;

        resp
    }
}

/// Convenience function to create an alert manager widget with a mutable vector of alerts.
///
/// # Parameters
/// - `alerts`: A mutable reference to a vector of `(AlertLevel, String)` tuples representing the current alerts.
///
/// # Returns
/// Returns an [`egui::Widget`] closure. When invoked, it returns an [`egui::Response`] for the alert manager area.
///
/// # Example
/// ```
/// # egui::__run_test_ui(|ui| {
/// # let mut alerts = vec![(egui_widget_ext::AlertLevel::Info, "Hello!".to_string())];
/// ui.add(egui_widget_ext::alert_manager(&mut alerts));
/// # });
/// ```
pub fn alert_manager<'a>(alerts: &'a mut Vec<(AlertLevel, String)>) -> impl Widget + 'a {
    move |ui: &mut Ui| AlertManager::new(alerts, "main").ui(ui)
}
