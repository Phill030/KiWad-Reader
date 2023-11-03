use eframe::{run_native, NativeOptions};
use window::Window;

mod window;
fn main() {
    let win_option = NativeOptions::default();
    run_native(
        "VoiceClient",
        win_option,
        Box::new(move |cc| Box::new(Window::new(cc))),
    )
    .unwrap();
}
