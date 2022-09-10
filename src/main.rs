#![windows_subsystem = "windows"]

use eframe::{
    egui::{self, FontData, FontDefinitions, Key},
    epaint::vec2,
};
use egui::epaint::FontFamily;
use rand::Rng;
use soloud::*;

fn main() {
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(490.0, 300.0)),
        max_window_size: Some(egui::vec2(1000.0, 600.0)),
        initial_window_pos: Some(egui::pos2(495f32, 215f32)),
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_size: Some(egui::vec2(900 as f32, 700 as f32)),
        // It would be hard work to make the app properly resizable for me
        resizable: false,
        transparent: true,
        multisampling: 0,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "Тренировка таблицы умножения",
        native_options,
        Box::new(|cc| Box::new(MultiplicationTraining::new(cc))),
    );
}
struct MultiplicationTraining {
    first_num: u64,
    second_num: u64,
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

        // Load sounds
        let sl = Soloud::default().unwrap();
        let mut score_sound = audio::Wav::default();
        score_sound
            .load_mem(include_bytes!("resources/score_point.wav"))
            .unwrap();
        let mut ouch_sound = audio::Wav::default();
        ouch_sound
            .load_mem(include_bytes!("resources/ouch.wav"))
            .unwrap();

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

        //Init default fonts to load custom along with default
        let mut fonts = FontDefinitions::default();

        //Load custom font
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

        //Use custom fonts
        cc.egui_ctx.set_fonts(fonts);

        //Initial `pixels_per_point`
        cc.egui_ctx.set_pixels_per_point(1.7f32);

        Self::default()
    }
}

impl eframe::App for MultiplicationTraining {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_pixels_per_point(1.7f32); //2.21234 //1.632 //previous variants

        egui::CentralPanel::default().show(ctx, |ui| {

            //Top heading
            ui.vertical_centered_justified(|ui| {
                ui.heading("Тренировка таблицы умножения");
                ui.separator();
            });

            // Dark/Light mode
            egui::global_dark_light_mode_switch(ui);

            //Main body
            ui.allocate_ui_with_layout(
                vec2(490f32, 255f32),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.add_space(219f32);
                    ui.strong(self.first_num.to_string());
                    ui.label("*");
                    ui.strong(self.second_num.to_string());
                    ui.label("=");
                    ui.add(egui::TextEdit::singleline(&mut self.user_input).desired_width(14.3f32))
                        .request_focus();

                    //Logic of application
                    if self.user_input.chars().count() > 2 {
                        self.user_input = self.user_input.chars().take(2).collect::<String>();
                    }
                    match self.user_input.trim().parse::<u64>() {
                        Ok(user_answer) => {
                            if ctx.input().key_pressed(Key::Enter) {
                                if user_answer == self.second_num * self.first_num {
                                    self.user_input = "".to_string();
                                    if self.max_combo < self.combo {
                                        self.max_combo = self.combo;
                                    }
                                    self.combo += 1;
                                    self.first_num = rand::thread_rng().gen_range(2..=9);
                                    self.second_num = rand::thread_rng().gen_range(2..=9);
                                    self.sl.play(&self.score_sound);
                                } else {
                                    self.user_input = "".to_string();
                                    if self.max_combo < self.combo {
                                        self.max_combo = self.combo;
                                    }
                                    self.combo = 0;
                                    self.try_count += 1;
                                    self.sl.play(&self.ouch_sound);
                                }
                                if self.combo == 20 {
                                    ui.output().open_url = Some(egui::output::OpenUrl {
                                        new_tab: true,
                                        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                                            .to_string(),
                                    });
                                }
                            }
                        }
                        Err(_) => (),
                    };
                },
            );

            //Stats
            ui.group(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(1f32);
                    ui.label(format!("Максимальный счёт: {}", self.max_combo));
                    ui.add_space(41.6f32);
                    ui.label("Ср. время!");
                    ui.add_space(45f32);
                    ui.label(format!("Счёт: {}", self.combo));
                    ui.add_space(44.6f32);
                    ui.label(format!("Попытка № {}", self.try_count));
                });
            });

        });
    }
}