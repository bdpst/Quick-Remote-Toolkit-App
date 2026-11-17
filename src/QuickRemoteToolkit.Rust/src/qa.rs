//! Explicitly feature-gated rendering evidence, never enabled in portable builds.
use eframe::{egui, icon_data::IconDataExt};
fn path(label: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var_os("QRT_QA_OUTPUT").expect("QA output required"))
        .join(format!("{label}.png"))
}
pub fn captured(label: &str) -> bool {
    path(label).exists()
}
pub fn inject_invalid_enter(ctx: &egui::Context, label: &'static str) {
    if std::env::var("QRT_QA_SCENE").ok().as_deref() != Some(&format!("{label}-error")) {
        return;
    }
    let id = egui::Id::new(("qa-input", label));
    let n = ctx.data_mut(|d| {
        let n = d.get_temp::<usize>(id).unwrap_or_default() + 1;
        d.insert_temp(id, n);
        n
    });
    if n == 4 {
        let modifiers = if label == "message" {
            egui::Modifiers::CTRL
        } else {
            egui::Modifiers::default()
        };
        ctx.input_mut(|i| {
            i.modifiers = modifiers;
            i.events.push(egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            });
        });
    }
}
pub fn capture(ctx: &egui::Context, label: &'static str) {
    if std::env::var_os("QRT_QA_OUTPUT").is_none() || captured(label) {
        return;
    }
    let counter_id = egui::Id::new(("qa-frames", label));
    let frames = ctx.data_mut(|d| {
        let n = d.get_temp::<usize>(counter_id).unwrap_or_default() + 1;
        d.insert_temp(counter_id, n);
        n
    });
    if frames >= 8 {
        let (inner, outer, focused) = ctx.input(|i| {
            (
                i.viewport().inner_rect,
                i.viewport().outer_rect,
                i.viewport().focused,
            )
        });
        let info = format!(
            "viewport={:?}; inner={inner:?}; outer={outer:?}; focused={focused:?}; keyboard={}; widget={:?}; dpi={}",
            ctx.viewport_id(),
            ctx.wants_keyboard_input(),
            ctx.memory(|m| m.focused()),
            ctx.pixels_per_point()
        );
        std::fs::write(path(label).with_extension("txt"), info).expect("QA metadata");
        // eframe 0.33 does not service Screenshot commands for immediate viewports.
        // Read the actual framebuffer in the last paint layer for root and dialogs.
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Debug,
            egui::Id::new(("qa-capture", label)),
        ))
        .add(egui::PaintCallback {
            rect: ctx.viewport_rect(),
            callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                move |info, painter| {
                    let image = painter.read_screen_rgba(info.screen_size_px);
                    let icon = egui::IconData {
                        width: image.width() as u32,
                        height: image.height() as u32,
                        rgba: image.as_raw().to_vec(),
                    };
                    std::fs::write(path(label), icon.to_png_bytes().expect("PNG encoding"))
                        .expect("QA image write");
                },
            )),
        });
    }
    ctx.request_repaint();
}
