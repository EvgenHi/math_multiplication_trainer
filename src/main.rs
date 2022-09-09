#![windows_subsystem = "windows"]

use eframe::egui::{self, FontData, FontDefinitions, output::OpenUrl};
use egui::epaint::FontFamily;
use rand::Rng;
use soloud::*;

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
    combo: u64,
    max_combo: u64,
    try_count: u64,
    average_time: f64,
    sl: Soloud,
    score_sound: Wav,
    ouch_sound: Wav,
}

impl Default for MultiplicationTraining {
    fn default() -> Self {
        
        let sl = Soloud::default().unwrap();
        let mut score_sound = audio::Wav::default();
        score_sound.load_mem(include_bytes!("resources/score_point.wav")).unwrap();
        let mut ouch_sound = audio::Wav::default();
        ouch_sound.load_mem(include_bytes!("resources/ouch.wav")).unwrap();
        MultiplicationTraining {
            first_num: rand::thread_rng().gen_range(2..=9),
            second_num: rand::thread_rng().gen_range(2..=9),
            user_input: format!(""),
            combo: 0,
            max_combo: 0,
            try_count: 1,
            average_time: 0f64,
            sl,
            score_sound,
            ouch_sound,
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
            "Undertale".to_owned(),
            FontData::from_static(include_bytes!("resources/Undertale-Battle-Font.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Undertale".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("Undertale".to_owned());
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
            });

            //ui.add_space(75f32);
            

            // ui.label("Cколько будет ");
            // ui.strong(self.first_num.to_string());
            // ui.label("*");
            // ui.strong(self.second_num.to_string());

            // if ui.button("📋").clicked() {
            //     ui.output().open_url= Some(OpenUrl {
            //         new_tab: true,
            //         url: "https://youtu.be/dQw4w9WgXcQ".to_string(),
            //     });
            // }

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(240f32);
                ui.strong(self.first_num.to_string());
                ui.label("*");
                ui.strong(self.second_num.to_string());
                ui.label("=");
                let text_edit = egui::TextEdit::singleline(&mut self.user_input);
                let response = text_edit.desired_width(13.3f32);
                ui.add(response);
                let user_answer: u8 = match self.user_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 3,
                };

                if  self.user_input.chars().count() as u8
                    >= length(self.first_num * self.second_num, 10)
                {
                    if user_answer == self.second_num * self.first_num {
                        self.sl.play(&self.score_sound);
                        self.first_num = rand::thread_rng().gen_range(2..=9);
                        self.second_num = rand::thread_rng().gen_range(2..=9);
                        self.combo += 1;
                        self.user_input = "".to_string();
                        if self.max_combo < self.combo {
                            self.max_combo = self.combo;
                        }
                    } else {
                        self.try_count += 1;
                        if self.max_combo < self.combo {
                            self.max_combo = self.combo;
                        }
                        self.sl.play(&self.ouch_sound);
                        self.combo = 0;
                        self.user_input = "".to_string();
                    }
                    if self.combo == 10 {
                         ui.output().open_url= Some(OpenUrl {
                             new_tab: true,
                             url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
                         });
                    }
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                
                ui.label(format!("Максимальный счёт: {}", self.max_combo));
                ui.label("Ср. время!");
                ui.label(format!("Счёт: {}", self.combo));                
                ui.label(format!("Попытка № {}", self.try_count));
            });
        });
    }
}

fn length(n: u8, base: u8) -> u8 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}
