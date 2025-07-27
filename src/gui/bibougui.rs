use std::string;

use egui::{text_selection::visuals, FontData, FontDefinitions, FontFamily};

use crate::oshandler::handler::BibouHandler;

pub struct BibouGui {
    alertcount: usize,
    sessiontimer: usize,
    handler: BibouHandler,
    sessionactive: bool,
    displayedimage: Option<String>,
}

impl Default for BibouGui {
    fn default() -> Self {
        BibouGui {
            alertcount: 0,
            sessiontimer: 0,
            handler: BibouHandler::default(),
            sessionactive: false,
            displayedimage: None,
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
                ui.add_space(10.0);
                if !self.sessionactive {
                    if ui.button("Démarrer une session").clicked() {
                        self.sessionactive = true;
                        self.displayedimage = Some("../../assets/gifs/bibou-papouilles1-unscreen.gif".to_string());
                    }
                } else {
                    if ui.button("Mettre fin à la session").clicked() {
                        self.sessionactive = false;
                    }
                }
                if let Some(image) = &self.displayedimage {
                    ui.image(image);
                }

            });
        });
    }
}