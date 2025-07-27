use egui::{text_selection::visuals, FontData, FontDefinitions, FontFamily};

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
        //Styling and custom fonts handling

        let mut visuals = egui::Visuals::dark();

        visuals.panel_fill = egui::Color32::from_rgb(91, 95, 151);

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
            //Styling
            ctx.set_fonts(fonts);
            ctx.set_visuals(visuals);


            ui.vertical_centered_justified(|ui| {
                ui.label(egui::RichText::new("Bibou Watcher").size(56.0));
            });
        });
    }
}