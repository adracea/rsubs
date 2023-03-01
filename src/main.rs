use std::fs::File;
use std::io::Read;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 780.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rsubs GUI",
        options,
        Box::new(|_cc| Box::new(Rsubs::default())),
    )
}

struct Rsubs {
    filename: String,
    file: String,
}

impl Default for Rsubs {
    fn default() -> Self {
        Self {
            filename: "".to_owned(),
            file: "".to_owned(),
        }
    }
}

impl eframe::App for Rsubs {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.filename = path.display().to_string();
                            File::open(self.filename.clone())
                                .unwrap()
                                .read_to_string(&mut self.file)
                                .unwrap();
                        }
                    }
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let file_name_label = ui.label("File Name: ");
                    ui.text_edit_singleline(&mut self.filename)
                        .labelled_by(file_name_label.id);
                });
                if !self.filename.is_empty() {
                    ui.horizontal(|ui| {
                        let file_label = egui::Label::new("File: ");
                        let text_editor =
                            egui::text_edit::TextEdit::multiline(&mut self.file).clip_text(true);
                        ui.add(file_label);
                        ui.add_sized([400.0, 600.0], text_editor.code_editor());
                    });
                }
            });
        });
    }
}
