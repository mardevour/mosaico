use eframe::egui;
use eframe::egui::FontData;
use eframe::egui::FontDefinitions;
use eframe::egui::FontFamily;
use egui::Visuals;
use egui::Color32;

pub fn get_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert("unscii".to_owned(),
    std::sync::Arc::new(
        FontData::from_static(include_bytes!("assets/fonts/unscii-16-full.ttf"))
    ));
    
    fonts.families.get_mut(&FontFamily::Proportional)
    .unwrap()
    .insert(0, "unscii".to_owned());
    fonts
}

pub struct MosaicoVisuals {
    dark_mode: bool,
    panel_fill: Color32,
    override_text_color: Color32,
    faint_bg_color: Color32,
    clip_rect_margin: f32,
}

impl From<MosaicoVisuals> for Visuals {
    fn from(mosaico: MosaicoVisuals) -> Self {
        let mut visuals = egui::Visuals::default();
        
        visuals.dark_mode = mosaico.dark_mode;
        visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
        visuals.faint_bg_color = egui::Color32::from_rgb(18, 18, 18);
        visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));
        visuals.clip_rect_margin = 0.0;

        visuals
    }
}

pub fn get_visuals() -> Visuals {
    let mosaico = MosaicoVisuals {
        dark_mode: true,
        panel_fill: egui::Color32::from_rgb(0, 0, 0),
        faint_bg_color: egui::Color32::from_rgb(18, 18, 18),
        override_text_color: egui::Color32::from_rgb(255, 255, 255),
        clip_rect_margin: 0.0,
    };
    mosaico.into()
}