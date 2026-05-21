pub mod core;
pub mod ui;

use ui::RMM;
use eframe::egui;

#[tokio::main]
async fn main() {
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

    let args: Vec<String> = std::env::args().collect();

    if !args.is_empty() {
        core::web::gamebanana_uri_handler(&args[0]).await.expect("Unable to handle uri");
    }
}
