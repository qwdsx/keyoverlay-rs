use app::App;

mod app;
mod key;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Keyboard Overlay",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
