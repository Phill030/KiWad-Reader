use std::ops::Add;

use crate::wad::{FileRecord, WadRework};
use eframe::{
    egui::{
        CentralPanel, CollapsingHeader, Layout, RichText, ScrollArea, SidePanel, TextEdit,
        TopBottomPanel, Ui,
    },
    emath::Align,
    epaint::FontId,
    App,
};

#[derive(Default)]
pub struct Window {
    file_search: String,
    wad: WadRework,
    wad_path: String,
    selected_record: String,
    selected_record_buffer: Vec<u8>,
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
                        match WadRework::new(&self.wad_path) {
                            Ok(wad) => {
                                self.wad = wad;

                                self.invalid_file_found = false;
                                self.file_search.clear();
                                self.selected_record.clear();
                                self.selected_record_buffer.clear();
                            }
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
                    ui.label(format!("{} Files", self.wad.file_count));
                })
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.set_min_width(300.0);

            ScrollArea::vertical().show(ui, |ui| {
                let files: Vec<&FileRecord> = self.wad.files.values().collect();

                if !files.is_empty() {
                    ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                        ui.label("Search");
                        ui.add(
                            TextEdit::singleline(&mut self.file_search).hint_text("Something.txt"),
                        );
                    });

                    let split_by = if self.wad_path.contains("/") {
                        "/"
                    } else {
                        "\\"
                    };

                    let tree = build_file_system_tree(
                        files.iter().map(|f| f.file_name.as_str()).collect(),
                        self.wad_path.split(split_by).last().unwrap().to_string(),
                    );
                    tree.display_tree(0, ui, self);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.selected_record_buffer.len() > 0 {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("{}", self.selected_record))
                            .font(FontId::proportional(20.0)),
                    );
                    ui.separator();

                    let buffer = String::from_utf8(self.selected_record_buffer.clone())
                        .unwrap_or(String::from("Error converting buffer to String"));
                    let mut content = buffer.as_str();

                    let multi_line =
                        TextEdit::multiline(&mut content).desired_width(ui.available_width());
                    ui.add(multi_line);
                });
            } else {
                if !self.selected_record.is_empty() {
                    // TODO: Run read_file only once
                    let wad = self.wad.clone();

                    let values = wad.files.values();
                    let collected = values
                        .filter(|v| v.file_name.ends_with(self.selected_record.as_str()))
                        .collect::<Vec<&FileRecord>>();
                    let file = collected.first().unwrap();
                    let content = self.wad.read_file(&file.file_name).unwrap();

                    let buff = if content.is_empty() {
                        b"Empty file".to_vec()
                    } else {
                        content
                    };

                    self.selected_record_buffer = buff;
                }
            }
        });
    }
}

#[derive(Debug)]
enum Item {
    File(String),
    Directory(String, Vec<Item>),
}

impl Item {
    fn display_tree(&self, indent: usize, ui: &mut Ui, wnd: &mut Window) {
        match self {
            Item::File(name) => {
                let icon = match name.split(".").last().unwrap() {
                    "png" | "dds" | "bmp" | "jpg" | "jpeg" => "📷",
                    "mp3" | "ogg" | "wav" => "🎵",
                    _ => "🗋",
                };

                if ui
                    .selectable_label(wnd.selected_record.eq(name), format!("{icon}  ").add(&name))
                    .clicked()
                {
                    wnd.selected_record = name.to_owned();
                    wnd.selected_record_buffer.clear();
                }
            }

            Item::Directory(name, items) => {
                CollapsingHeader::new(String::from("🗀  ").add(name))
                    .default_open(false)
                    .show(ui, |ui| {
                        for item in items {
                            item.display_tree(indent + 1, ui, wnd);
                        }
                    });
            }
        }
    }
}

fn add_file_to_tree(current: &mut Item, path: &str) {
    let components: Vec<&str> = path.split('/').collect();
    match components.len() {
        1 => {
            if let Item::Directory(_, children) = current {
                children.push(Item::File(path.to_string()));
            }
        }
        _ => {
            if let Item::Directory(_, children) = current {
                let dir_name = components[0].to_string();
                if let Some(child) = children.iter_mut().find(|child| {
                    if let Item::Directory(name, _) = child {
                        name == &dir_name
                    } else {
                        false
                    }
                }) {
                    add_file_to_tree(child, &path[(dir_name.len() + 1)..]);
                } else {
                    let mut new_dir = Item::Directory(dir_name.clone(), vec![]);
                    add_file_to_tree(&mut new_dir, &path[(dir_name.len() + 1)..]);
                    children.push(new_dir);
                }
            }
        }
    }
}

fn build_file_system_tree(paths: Vec<&str>, wad_name: String) -> Item {
    let mut root = Item::Directory(wad_name, vec![]);
    for path in paths {
        add_file_to_tree(&mut root, path);
    }
    root
}
