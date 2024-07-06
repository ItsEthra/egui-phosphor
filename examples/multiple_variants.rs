use egui::ScrollArea;
use egui_phosphor::{bold, fill, light, regular, thin};

fn main() {
    eframe::run_native(
        "egui-phosphor demo",
        Default::default(),
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
    .unwrap();
}

struct Demo {}

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "phosphor-thin".into(),
            egui_phosphor::Variant::Thin.font_data(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("phosphor-thin".into()),
            vec!["Ubuntu-Light".into(), "phosphor-thin".into()],
        );

        fonts.font_data.insert(
            "phosphor-light".into(),
            egui_phosphor::Variant::Light.font_data(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("phosphor-light".into()),
            vec!["Ubuntu-Light".into(), "phosphor-light".into()],
        );

        fonts.font_data.insert(
            "phosphor".into(),
            egui_phosphor::Variant::Regular.font_data(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("phosphor".into()),
            vec!["Ubuntu-Light".into(), "phosphor".into()],
        );

        fonts.font_data.insert(
            "phosphor-bold".into(),
            egui_phosphor::Variant::Bold.font_data(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("phosphor-bold".into()),
            vec!["Ubuntu-Light".into(), "phosphor-bold".into()],
        );

        fonts.font_data.insert(
            "phosphor-fill".into(),
            egui_phosphor::Variant::Fill.font_data(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("phosphor-fill".into()),
            vec!["Ubuntu-Light".into(), "phosphor-fill".into()],
        );

        cc.egui_ctx.set_fonts(fonts);

        Self {}
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let examples = [
            ("phosphor-thin", thin::FILE_CODE),
            ("phosphor-light", light::FILE_CODE),
            ("phosphor", regular::FILE_CODE),
            ("phosphor-bold", bold::FILE_CODE),
            ("phosphor-fill", fill::FILE_CODE),
        ];
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().id_source("loop").show(ui, |ui| {
                let mut iter = examples.iter();
                while let Some((family, icon)) = iter.next() {
                    ui.horizontal(|ui| {
                        show_demo_tile(ui, family, icon);
                        if let Some((next_family, next_icon)) = iter.next() {
                            show_demo_tile(ui, next_family, next_icon);
                        }
                    });
                }
            });
        });
    }
}

fn show_demo_tile(ui: &mut egui::Ui, family: &str, icon: &str) {
    ui.vertical(|ui| {
        ui.heading(family);
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.vertical(|ui| {
                for size in [16.0, 32.0, 48.0] {
                    let demo_text = format!("{} {icon}", family, icon = icon);
                    ui.label(
                        egui::RichText::new(&demo_text)
                            .family(egui::FontFamily::Name(family.into()))
                            .size(size),
                    );
                }
            });
        });
    });
}
