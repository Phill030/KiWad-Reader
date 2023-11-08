use std::ops::Add;

use crate::wad::{FileRecord, WadRework};
use eframe::{
    egui::{
        CentralPanel, CollapsingHeader, Layout, RichText, ScrollArea, SidePanel, TextEdit,
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
            ui.set_min_width(270.0);

            ScrollArea::vertical().show(ui, |ui| {
                if !self.files.is_empty() {
                    ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                        ui.label("Search");
                        ui.add(
                            TextEdit::singleline(&mut self.file_search).hint_text("Something.txt"),
                        );
                    });

                    let tree = build_file_system_tree(
                        self.files.iter().map(|f| f.file_name.as_str()).collect(),
                        "CSR.wad".to_string(),
                    );
                    tree.display_tree(0, ui, self);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.selected_record_content.len() > 0 {
                ui.text_edit_multiline(&mut format!("{:#?}", self.selected_record_content));
            } else {
                let file = self
                    .files
                    .iter()
                    .filter(|f| f.file_name.ends_with(f.file_name().as_str()))
                    .collect::<Vec<&FileRecord>>()
                    .first()
                    .unwrap();
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
                if ui
                    .selectable_label(false, String::from("🗋  ").add(&name))
                    .clicked()
                {
                    wnd.selected_record = name.to_owned();
                    wnd.selected_record_content = String::from("");
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
