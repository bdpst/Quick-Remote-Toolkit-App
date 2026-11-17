#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod columns;
mod design;
mod icon;
mod models;
mod native;
mod selection;
mod services;
mod ui;
pub use icon::application_icon;
#[cfg(feature = "ui-qa")]
mod qa;

fn main() -> eframe::Result {
    let icon = application_icon();
    eframe::run_native(
        concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
        eframe::NativeOptions {
            centered: true,
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([1500.0, 920.0])
                .with_min_inner_size([1200.0, 760.0])
                .with_decorations(false)
                .with_icon(icon),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(ui::Toolkit::new(cc)))),
    )
}
