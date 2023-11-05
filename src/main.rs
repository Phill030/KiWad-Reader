use eframe::{
    egui::{Style, Visuals},
    run_native, NativeOptions,
};
use window::Window;

pub mod wad;
mod window;

fn main() {
    let win_option = NativeOptions::default();
    run_native(
        "VoiceClient",
        win_option,
        Box::new(move |cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(Window::new(cc))
        }),
    )
    .unwrap();
}
