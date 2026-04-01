use egui::{Color32, CornerRadius, Stroke, style::WidgetVisuals};
use std::sync::atomic::{AtomicBool, Ordering};

static DARK_MODE: AtomicBool = AtomicBool::new(true);

pub fn is_dark() -> bool {
    DARK_MODE.load(Ordering::Relaxed)
}

pub fn set_dark(dark: bool) {
    DARK_MODE.store(dark, Ordering::Relaxed);
}

pub fn toggle() {
    DARK_MODE.fetch_xor(true, Ordering::Relaxed);
}

fn dl(dark: Color32, light: Color32) -> Color32 {
    if is_dark() { dark } else { light }
}

fn wv(bg: Color32, fg: Color32, stroke: Stroke, cr: CornerRadius) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: bg,
        weak_bg_fill: bg,
        bg_stroke: stroke,
        fg_stroke: Stroke::new(1.0, fg),
        corner_radius: cr,
        expansion: 0.0,
    }
}

pub fn apply_to_egui(ctx: &egui::Context) {
    let dark = is_dark();
    let mut v = if dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    let cr = CornerRadius::same(6);
    let bdr = Stroke::new(0.5, border());

    v.panel_fill = panel_bg();
    v.window_fill = dialog_bg();
    v.window_stroke = bdr;
    v.faint_bg_color = button_inactive_bg();
    v.extreme_bg_color = Color32::from_gray(if dark { 6 } else { 252 });
    v.code_bg_color = input_bg();
    v.hyperlink_color = accent();
    v.selection.bg_fill = selected_bg();
    v.selection.stroke = Stroke::new(1.0, accent());
    v.popup_shadow = egui::epaint::Shadow {
        spread: 0,
        blur: 12,
        offset: [0, 4],
        color: Color32::from_black_alpha(if dark { 50 } else { 20 }),
    };

    v.widgets.noninteractive = wv(Color32::TRANSPARENT, text_muted(), Stroke::NONE, cr);
    v.widgets.inactive = wv(button_inactive_bg(), icon_color(), bdr, cr);
    v.widgets.hovered = wv(hover_bg(), icon_hover_color(), bdr, cr);
    v.widgets.active = wv(selected_bg(), text(), Stroke::new(1.0, accent()), cr);
    v.widgets.open = wv(selected_bg(), text(), bdr, cr);

    ctx.set_visuals(v);
}

pub fn text() -> Color32 { dl(Color32::from_rgb(224, 224, 230), Color32::from_rgb(28, 28, 30)) }
pub fn text_muted() -> Color32 { dl(Color32::from_rgb(132, 132, 142), Color32::from_rgb(99, 99, 102)) }
pub fn border() -> Color32 { dl(Color32::from_white_alpha(12), Color32::from_black_alpha(18)) }
pub fn accent() -> Color32 { dl(Color32::from_rgb(115, 125, 150), Color32::from_rgb(90, 120, 165)) }
pub fn hover_bg() -> Color32 { dl(Color32::from_white_alpha(8), Color32::from_black_alpha(8)) }
pub fn selected_bg() -> Color32 { dl(Color32::from_white_alpha(16), Color32::from_black_alpha(14)) }
pub fn button_inactive_bg() -> Color32 { dl(Color32::from_white_alpha(6), Color32::from_black_alpha(6)) }
pub fn icon_color() -> Color32 { dl(Color32::from_rgb(156, 156, 168), Color32::from_rgb(72, 72, 74)) }
pub fn icon_hover_color() -> Color32 { dl(Color32::from_rgb(170, 170, 180), Color32::from_rgb(44, 44, 46)) }
pub fn separator_color() -> Color32 { dl(Color32::from_white_alpha(10), Color32::from_black_alpha(12)) }
pub fn input_bg() -> Color32 { dl(Color32::from_black_alpha(30), Color32::from_white_alpha(200)) }
pub fn input_border() -> Color32 { dl(Color32::from_white_alpha(14), Color32::from_black_alpha(18)) }
pub fn input_border_hover() -> Color32 { dl(Color32::from_white_alpha(22), Color32::from_black_alpha(30)) }
pub fn input_text_color() -> Color32 { text() }
pub fn backdrop() -> Color32 { dl(Color32::from_black_alpha(120), Color32::from_black_alpha(80)) }
pub fn section_label_color() -> Color32 { text_muted() }
pub fn dialog_border() -> Color32 { border() }
pub fn primary_button_bg() -> Color32 { accent() }
pub fn secondary_button_bg() -> Color32 { dl(Color32::from_white_alpha(8), Color32::from_black_alpha(8)) }
pub fn secondary_button_text() -> Color32 { text_muted() }

pub fn panel_bg() -> Color32 {
    dl(
        Color32::from_rgba_premultiplied(22, 22, 30, 220),
        Color32::from_rgba_premultiplied(244, 244, 248, 220),
    )
}

pub fn canvas_bg() -> Color32 {
    dl(Color32::from_rgb(18, 18, 24), Color32::from_rgb(250, 250, 252))
}

pub fn dialog_bg() -> Color32 {
    dl(
        Color32::from_rgba_premultiplied(28, 28, 36, 240),
        Color32::from_rgba_premultiplied(248, 248, 252, 245),
    )
}
