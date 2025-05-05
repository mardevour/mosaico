use eframe::egui;
use eframe::egui::RichText;
use crate::egui::ImageButton;
use crate::egui::Image;
mod layers;

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

        // top bar panel
        egui::TopBottomPanel::top("topBar").show(ctx, |ui| {
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

        // left side panel
        egui::SidePanel::left("leftPanel").resizable(false).min_width(50.0).max_width(50.0).show(ctx, |ui| {
            ui.label("Tools");

            let tool_brush = ui.add(ImageButton::new(Image::new(egui::include_image!("assets/icons/brush_12x12.png"))
            .max_size(egui::Vec2{ x: 24.0, y: 24.0 })));
            if tool_brush.clicked() {
            }

            let tool_eraser = ui.add(ImageButton::new(Image::new(egui::include_image!("assets/icons/eraser_12x12.png"))
            .max_size(egui::Vec2{ x: 24.0, y: 24.0 })));
            if tool_eraser.clicked() {
            }

            let tool_eraser = ui.add(ImageButton::new(Image::new(egui::include_image!("assets/icons/eraser_12x12.png"))
            .max_size(egui::Vec2{ x: 24.0, y: 24.0 })));
            if tool_eraser.clicked() {
            }

            let tool_eraser = ui.add(ImageButton::new(Image::new(egui::include_image!("assets/icons/eraser_12x12.png"))
            .max_size(egui::Vec2{ x: 24.0, y: 24.0 })));
            if tool_eraser.clicked() {
            }

            ui.separator();
            ui.label("more things here...");
        });

        // right side panel
        egui::SidePanel::right("rightPanel").resizable(true).min_width(200.0).max_width(600.0).show(ctx, |ui| {
            ui.separator();
            ui.label("even more things here...");

            // layers panel
            egui::TopBottomPanel::bottom("layersPanel").resizable(true).show_inside(ui, |ui| {
                ui.label(RichText::new("Layers").heading());
                ui.separator();

                let mut opacity: f32 = 100.0;
                egui::Grid::new("layersOptions").show(ui, |ui| {
                    ui.label("Opacity");
                    ui.add(egui::Slider::new(&mut opacity, 0.0..=100.0));
                    ui.end_row();
                });
                

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.separator();
                        ui.expand_to_include_y(ui.available_rect_before_wrap().bottom());
                        egui::Grid::new("layersContainer").striped(true).show(ui, |ui| {
                            ui.checkbox(&mut true, "👁️");
                            ui.label("nombre de la capa");
                            ui.end_row();
                        });
                    });
                });
            })

        });

        // central canvas area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Hello World").heading());
        });
    }
}