/// A function that creates a toggle switch UI element.
///
/// # Parameters
/// - `ui`: A mutable reference to the `egui::Ui` object, which represents the current UI context.
/// - `on`: A mutable reference to a boolean value that represents the state of the toggle switch.
///         If `true`, the toggle switch is "on"; if `false`, it is "off".
///
/// # Returns
/// - An `egui::Response` object that contains information about the interaction with the toggle switch.
///
/// # Behavior
/// - The function allocates space for the toggle switch in the UI and handles user interaction.
/// - When the toggle switch is clicked, the `on` state is toggled, and the response is marked as changed.
/// - The function also handles the visual representation of the toggle switch, including its background
///   and the animated movement of the toggle circle.
pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

/// A wrapper function that creates a toggle switch widget.
///
/// # Parameters
/// - `on`: A mutable reference to a boolean value that represents the state of the toggle switch.
///         If `true`, the toggle switch is "on"; if `false`, it is "off".
///
/// # Returns
/// - An implementation of the `egui::Widget` trait that can be used to add the toggle switch to a UI.
///
/// # Behavior
/// - This function returns a closure that calls `toggle_ui` with the provided `on` state and the UI context.
/// - It allows the toggle switch to be easily embedded in an `egui` UI layout.
pub fn toggle_switch(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
