use eframe::egui;
use crate::egui::ImageButton;
use crate::egui::Image;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("mosaico", native_options, Box::new(|cc| Ok(Box::new(Mosaico::new(cc)))));
}

#[derive(Default)]
struct Mosaico {}

impl Mosaico {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Mosaico {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(&ctx);
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        todo!();
                    }
                    if ui.button("Open").clicked() {
                        todo!();
                    }
                    if ui.button("Save").clicked() {
                        todo!();
                    }
                    if ui.button("Save as...").clicked() {
                        todo!();
                    }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("todo!").clicked() {
                            todo!();
                        }
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("todo!").clicked() {
                            todo!();
                        }
                    });
            })
            
        });

        egui::SidePanel::left("leftpanel").resizable(true).show(ctx, |ui| {
            ui.separator();
            ui.label("Tools");
            let tool_brush = ui.add(ImageButton::new(Image::new(egui::include_image!("assets/icons/brush_12x12.png"))
            .max_size(egui::Vec2{ x: 24.0, y: 24.0 })));
            if tool_brush.clicked() {
            }
         });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}