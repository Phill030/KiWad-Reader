use eframe::{
    egui::{CentralPanel, SidePanel},
    App,
};

#[derive(Default)]
pub struct Window {}

impl Window {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl App for Window {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        draw_left_panel(&ctx);
        draw_center_panel(&ctx);
    }
}

fn draw_left_panel(ctx: &eframe::egui::Context) {
    SidePanel::left("left_panel").show(ctx, |ui| {
        //
    });
}
fn draw_center_panel(ctx: &eframe::egui::Context) {
    CentralPanel::default().show(ctx, |ui| {});
}
