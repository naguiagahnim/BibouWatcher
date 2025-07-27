use WorkBibou::gui::bibougui::BibouGui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_title("Talos"),
        ..Default::default()
    };

    eframe::run_native(
        "Talos",
        native_options,
        Box::new(|_cc| Ok(Box::new(BibouGui::default()))),
    )
}