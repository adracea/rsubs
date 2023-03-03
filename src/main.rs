use std::fs;
use std::fs::File;

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
        Box::new(|_cc| Box::<Rsubs>::default()),
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
impl Rsubs {
    fn save_file(&self) {
        println!("{}", self.filename.clone());
        fs::write(self.filename.clone(), self.file.clone()).unwrap();
    }
    fn saveas_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            self.filename = path.display().to_string();
            fs::write(self.filename.clone(), self.file.clone()).unwrap();
        }
    }
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.filename = path.display().to_string();
            self.file = fs::read_to_string(self.filename.clone()).unwrap();
        }
    }
}

impl eframe::App for Rsubs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let save_shortcut = egui::KeyboardShortcut::new(
            egui::Modifiers {
                alt: false,
                ctrl: false,
                shift: false,
                mac_cmd: true,
                command: true,
            },
            egui::Key::S,
        );
        let saveas_shortcut = egui::KeyboardShortcut::new(
            egui::Modifiers {
                alt: false,
                ctrl: false,
                shift: true,
                mac_cmd: true,
                command: true,
            },
            egui::Key::S,
        );
        let open_shortcut = egui::KeyboardShortcut::new(
            egui::Modifiers {
                alt: false,
                ctrl: false,
                shift: false,
                mac_cmd: true,
                command: true,
            },
            egui::Key::O,
        );
        let save_btn = egui::Button::new("Save").shortcut_text(ctx.format_shortcut(&save_shortcut));
        let saveas_btn =
            egui::Button::new("Save As...").shortcut_text(ctx.format_shortcut(&saveas_shortcut));
        let open_btn = egui::Button::new("Open").shortcut_text(ctx.format_shortcut(&open_shortcut));
        if ctx.input_mut(|i| i.consume_shortcut(&open_shortcut)) {
            self.open_file();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&save_shortcut)) {
            self.save_file();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&saveas_shortcut)) {
            self.saveas_file();
        }
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if (ui.add(open_btn)).clicked() {
                        self.open_file();
                    }
                    if ui.add(save_btn).clicked()
                        && !self.filename.is_empty()
                        && File::open(self.filename.clone()).is_ok()
                    {
                        self.save_file();
                    }
                    if ui.add(saveas_btn).clicked() {
                        self.saveas_file();
                    }
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let file_name_label = ui.label("File Name: ");
                    ui.text_edit_singleline(&mut self.filename)
                        .labelled_by(file_name_label.id);
                });
                if !self.filename.is_empty() {
                    let file_label = egui::Label::new("File: ");
                    let line_nr = self.file.lines().count() + 1;
                    ui.add(file_label);
                    ui.group(|ui| {
                        egui::ScrollArea::both()
                            .max_width(800.0)
                            .max_height(700.0)
                            .min_scrolled_height(700.0)
                            .show(ui, |ui| {
                                ui.horizontal_top(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_max_size([36.0, 12.0].into());
                                        let mut text = "".to_string();
                                        for row in 1..line_nr + 1 {
                                            text += &(row.to_string() + "\n");
                                        }
                                        ui.vertical_centered_justified(|ui| {
                                            ui.label(egui::RichText::new(text).code().size(12.0));
                                        });
                                    });
                                    let mut layouter =
                                        |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                                            let mut layout_job: egui::text::LayoutJob =
                                                egui::text::LayoutJob::simple(
                                                    string.to_owned(),
                                                    egui::FontId::monospace(12.0),
                                                    egui::Color32::from_rgb(255, 255, 255),
                                                    f32::INFINITY,
                                                );
                                            layout_job.wrap.max_width = f32::INFINITY;
                                            ui.fonts(|f| f.layout_job(layout_job))
                                        };
                                    let text_editor =
                                        egui::text_edit::TextEdit::multiline(&mut self.file)
                                            .code_editor()
                                            .hint_text("Code Here")
                                            .min_size([600.0, 700.0].into())
                                            .desired_width(f32::INFINITY)
                                            .layouter(&mut layouter);
                                    ui.add(text_editor.clip_text(false).code_editor());
                                });
                            });
                    });
                }
            });
        });
    }
}
