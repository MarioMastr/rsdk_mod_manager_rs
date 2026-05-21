pub mod core;
pub mod ui;

use ui::RMM;
use eframe::egui;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([666.0, 585.0])
            .with_min_inner_size([666.0, 585.0]),

        ..Default::default()
    };
    let _ = eframe::run_native(
        "RSDK Mod Manager",
        options,
        Box::new(|cc| {
            Ok(Box::new(RMM::new(cc)))
        })
    );
}
