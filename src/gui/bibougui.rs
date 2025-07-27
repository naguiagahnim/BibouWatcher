use egui::{FontData, FontDefinitions, FontFamily};

use crate::oshandler::handler::BibouHandler;

pub struct BibouGui {
    alertcount: usize,
    sessiontimer: usize,
    handler: BibouHandler,
}

impl Default for BibouGui {
    fn default() -> Self {
        BibouGui {
            alertcount: 0,
            sessiontimer: 0,
            handler: BibouHandler::default()
        }
    }
}

impl eframe::App for BibouGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("my_font".to_owned(),
        std::sync::Arc::new(
            FontData::from_static(include_bytes!("../../assets/fonts/BagelFatOne-Regular.ttf"))
        )
        );

        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_fonts(fonts);
            ui.vertical_centered_justified(|ui| {
                ui.heading("Bibou Watcher");
                ui.separator();
            });
        });
    }
}