#[cfg(all(feature = "iced", feature = "egui"))]
compile_error!("Iced GUI and egui GUI cannot be compiled together");

pub mod ui;
pub mod core;

fn main() {
    env_logger::init();

    cfg_if::cfg_if! {
        if #[cfg(feature = "egui")] {
            let options = eframe::NativeOptions {
                viewport: eframe::egui::ViewportBuilder::default()
                    .with_inner_size([666.0, 585.0])
                    .with_min_inner_size([666.0, 585.0]),

                ..Default::default()
            };
            let _ = eframe::run_native(
                "RSDK Mod Manager",
                options,
                Box::new(|cc| {
                    Ok(Box::new(ui::egui::RMM::new(cc)))
                })
            );
        } else if #[cfg(feature = "iced")] {
            iced::application(ui::iced::RMM::new, ui::iced::RMM::update, ui::iced::RMM::view)
                .theme(iced::Theme::CatppuccinMocha)
                .title(ui::iced::RMM::title)
                .run().expect("Unable to run application");
        }
    }
}
