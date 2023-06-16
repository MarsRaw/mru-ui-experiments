use marsrawutils_ui::gui::MruApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(1920., 1080.0)),
        ..Default::default()
    };
    eframe::run_native("MRU-UI", options, Box::new(|_cc| Box::<MruApp>::default()))
}
