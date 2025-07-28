use egui::{include_image, FontData, FontDefinitions, FontFamily};
use crate::oshandler::handler::BibouHandler;

const BIBOU_GIF: egui::ImageSource<'_> = include_image!("../../assets/gifs/bibou.gif");
const SHARK1: egui::ImageSource<'_> = include_image!("../../assets/images/shark.png");


pub struct BibouGui {
    alertcount: usize,
    sessiontimer: usize,
    handler: BibouHandler,
    sessionactive: bool,
}

impl Default for BibouGui {
    fn default() -> Self {
        BibouGui {
            alertcount: 0,
            sessiontimer: 0,
            handler: BibouHandler::default(),
            sessionactive: false,
        }
    }
}

impl eframe::App for BibouGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Handle image support
        egui_extras::install_image_loaders(ctx);

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
                    }
                    ui.add(egui::Image::new(SHARK1).max_width(1000.0));
                } else {
                    if ui.button("Mettre fin à la session").clicked() {
                        self.sessionactive = false;
                    }
                    ui.add(egui::Image::new(BIBOU_GIF).max_width(300.0));
                    //Gestion de l'os à faire
                    ui.add_space(10.0);
                    self.sessiontimer += 1;

                    let secondes = self.sessiontimer / 60;
                    let minutes = secondes / 60;
                    let heures = minutes / 60;

                    let time_string = format!("{:02}:{:02}:{:02}", heures, minutes % 60, secondes % 60);

                    ui.label(egui::RichText::new(time_string).size(30.0));
                }
            });
        });
    }
}