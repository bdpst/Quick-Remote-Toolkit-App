//! Test host only: actual application UI with synthetic states and framebuffer capture.
//! Not included by build.ps1 or the portable archive.
mod columns;
mod design;
mod icon;
mod models;
mod native;
mod qa;
mod selection;
mod services;
mod ui;
use eframe::egui;
pub use icon::application_icon;

struct Host {
    app: ui::Toolkit,
    scene: String,
}
impl eframe::App for Host {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.update(ctx, frame);
        qa::capture(ctx, "main");
        let child = match self.scene.as_str() {
            "message" | "message-error" => Some("message"),
            "admin" | "admin-error" => Some("admin"),
            _ => None,
        };
        if qa::captured("main") && child.is_none_or(qa::captured) {
            let output =
                std::path::PathBuf::from(std::env::var_os("QRT_QA_OUTPUT").expect("QA output"));
            std::fs::write(
                output.join("state.json"),
                serde_json::to_vec_pretty(&self.app.qa_state()).expect("QA state"),
            )
            .expect("QA state write");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
fn main() -> eframe::Result {
    let output = std::env::var_os("QRT_QA_OUTPUT").expect("QRT_QA_OUTPUT required");
    std::fs::create_dir_all(output).expect("QA output directory");
    // Require an explicit isolated profile; never use the person's normal settings.
    assert!(
        std::env::var_os("QRT_QA_PROFILE").is_some_and(|v| Some(v) == std::env::var_os("APPDATA"))
    );
    let scene = std::env::var("QRT_QA_SCENE").unwrap_or_else(|_| "main".into());
    let width = std::env::var("QRT_QA_WIDTH")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1500.);
    let height = std::env::var("QRT_QA_HEIGHT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(920.);
    eframe::run_native(
        concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
        eframe::NativeOptions {
            centered: true,
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([width, height])
                .with_min_inner_size([1200., 760.])
                .with_decorations(false)
                .with_icon(application_icon()),
            ..Default::default()
        },
        Box::new(move |cc| {
            let mut app = ui::Toolkit::new(cc);
            app.qa_scene(&scene, &cc.egui_ctx);
            Ok(Box::new(Host { app, scene }))
        }),
    )
}
