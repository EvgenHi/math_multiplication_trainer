use eframe::egui::{self, FontData, FontDefinitions};
use egui::epaint::FontFamily;
use rand::Rng;

fn main() {
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(500.0, 300.0)),
        max_window_size: Some(egui::vec2(1000.0, 600.0)),
        initial_window_pos: None, //Some(egui::pos2(0f32, 0f32))
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_size: Some(egui::vec2(900 as f32, 700 as f32)),
        resizable: true,
        transparent: true,
        multisampling: 0,
        stencil_buffer: 0,
        ..Default::default()
    };
    eframe::run_native(
        "Тренировка таблицы умножения",
        native_options,
        Box::new(|cc| Box::new(MultiplicationTraining::new(cc))),
    );
}
struct MultiplicationTraining {
    first_num: u8,
    second_num: u8,
    user_input: String,
    is_right: String,
    check: bool,
}

impl Default for MultiplicationTraining {
    fn default() -> Self {
        MultiplicationTraining {
            first_num: rand::thread_rng().gen_range(2..=9),
            second_num: rand::thread_rng().gen_range(2..=9),
            user_input: format!(""),
            is_right: format!(""),
            check: false,
        }
    }
}

impl MultiplicationTraining {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "sunday".to_owned(),
            FontData::from_static(include_bytes!("Undertale-Battle-Font.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "sunday".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("sunday".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_pixels_per_point(1.6); //2.21234 //1.632
        Self::default()
    }
}

impl eframe::App for MultiplicationTraining {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Тренировка таблицы умножения");
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.label(format!(
                        "Сколько будет {} * {}?",
                        self.first_num, self.second_num
                    ));
                    ui.checkbox(&mut self.check, "Checkbox");
                    if ui.text_edit_singleline(&mut self.user_input).lost_focus() {
                        let user_answer: u8 = match self.user_input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => 3,
                        };
                        if user_answer == self.second_num * self.first_num {
                            self.is_right = "Верно!".to_string();
                            self.first_num = rand::thread_rng().gen_range(2..=9);
                            self.second_num = rand::thread_rng().gen_range(2..=9);
                        } else {
                            self.is_right = "Неверно!".to_string();
                            
                        }
                    };
                });
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.label("Count!");
                });
            });
        });
    }
}