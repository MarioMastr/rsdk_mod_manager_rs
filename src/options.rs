use eframe::egui;

#[derive(PartialEq)]
pub struct Options {
    save_path: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            save_path: String::new(),
        }
    }
}

impl Options {
    pub fn ui(&mut self, _ui: &mut egui::Ui) {

    }
}