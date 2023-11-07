use std::ops::Add;

use crate::wad::{FileRecord, WadRework};
use eframe::{
    egui::{
        CentralPanel, CollapsingHeader, Id, Layout, RichText, ScrollArea, SidePanel, TextEdit,
        TopBottomPanel, Ui,
    },
    emath::Align,
    App,
};

#[derive(Default)]
pub struct Window {
    file_search: String,
    files: Vec<FileRecord>,
    wad_path: String,
    selected_record: String,
    selected_record_content: String,
    invalid_file_found: bool,
}

impl Window {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl App for Window {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                ui.label("Path:");
                ui.add(TextEdit::singleline(&mut self.wad_path).hint_text("Path of the .wad file"));
                if ui.button("Open file").clicked() {
                    if !self.wad_path.is_empty() {
                        match std::fs::read(&self.wad_path) {
                            Ok(mut buffer) => match WadRework::new(&mut buffer) {
                                Ok(wad) => {
                                    self.files = wad.files.values().cloned().collect();
                                    for ele in &self.files {
                                        println!("{}", ele.file_name);
                                    }
                                    self.invalid_file_found = false;
                                }
                                Err(_) => self.invalid_file_found = true,
                            },
                            Err(_) => self.invalid_file_found = true,
                        }
                    }
                }

                if self.invalid_file_found {
                    ui.label(
                        RichText::new("Invalid file provided!").color(ui.visuals().error_fg_color),
                    );
                }

                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    ui.label(format!("{} Files", self.files.len()));
                })
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.set_min_width(200.0);

            ScrollArea::vertical().show(ui, |ui| {
                if !self.files.is_empty() {
                    ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                        ui.label("Search");
                        ui.add(
                            TextEdit::singleline(&mut self.file_search).hint_text("Something.txt"),
                        );
                    });

                    CollapsingHeader::new("CSR.wad")
                        .default_open(self.files.len() > 0)
                        .show(ui, |ui| {
                            //
                        });
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.selected_record_content.len() > 0 {
                ui.text_edit_multiline(&mut format!("{:#?}", self.selected_record_content));
            } else {
                self.selected_record_content = self.selected_record.clone();
            }
        });
    }
}

fn recursive_show(ui: &mut Ui, input: &FileRecord, full_file_name: &String, wnd: &mut Window) {
    if input.file_name.contains("/") {
        let path = &input.file_name.split("/").collect::<Vec<&str>>();
        let file_name = path.first().unwrap().to_string();
        let new_path = &input.file_name[file_name.len() + 1..];

        CollapsingHeader::new(String::from("🗀  ").add(&file_name))
            .id_source(Id::with(ui.id(), full_file_name))
            .default_open(false)
            .show(ui, |ui| {
                let record = FileRecord {
                    file_name: new_path.to_string(),
                    ..*input
                };

                recursive_show(ui, &record, full_file_name, wnd);
            });
    } else {
        if ui
            .selectable_label(false, String::from("🗋  ").add(&input.file_name))
            .clicked()
        {
            wnd.selected_record = full_file_name.to_owned();
            wnd.selected_record_content = String::from("");
        }
    }
}

// Input:
// QuestData/KR/KR-LAST-C18-001.xml
// QuestData/WC/WC-ICE-C04-001.xml
// QuestArcData/KT-CRY5-C01.xml

// Define a custom folder structure
