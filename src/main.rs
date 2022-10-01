#![windows_subsystem = "windows"]

use eframe::{
    egui::{self, FontData, FontDefinitions, Key, Label},
    epaint::vec2, IconData,
};
use egui::epaint::FontFamily;
use rand::Rng;
use soloud::*;

fn main() {

    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(715.0, 300.0)),
        max_window_size: None, //Some(egui::vec2(1000.0, 600.0)),
        initial_window_pos: Some(egui::pos2(548f32, 215f32)),
        initial_window_size: Some(egui::vec2(900 as f32, 600 as f32)),
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: Some(IconData {
            rgba: include_bytes!("resources/icon.rgba").to_vec(),
            width: 32,
            height: 32,

        }),
        resizable: true,
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
    volume: f32,
}

impl Default for MultiplicationTraining {
    fn default() -> Self {

        // Load sounds
        let mut sl = Soloud::default().unwrap();
        let mut score_sound = audio::Wav::default();
        score_sound
            .load_mem(include_bytes!("resources/score_point.wav"))
            .unwrap();
        let mut ouch_sound = audio::Wav::default();
        ouch_sound
            .load_mem(include_bytes!("resources/ouch.wav"))
            .unwrap();

        sl.set_global_volume(0.32f32);

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
            volume: 40f32,
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
        cc.egui_ctx.set_pixels_per_point(1.5f32);

        Self::default()
    }
}

impl eframe::App for MultiplicationTraining {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5f32);

        // Next update will contain colored themes
        /*
        let my_frame = egui::containers::Frame {
            inner_margin: Margin::symmetric(10f32, 10f32),
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
            fill: Color32::DARK_RED,
            //stroke: egui::Stroke::new(2.0, Color32::GOLD),
            ..Default::default()
        };
        */

        egui::CentralPanel::default()
            .show(ctx, |ui| {

                //Top heading
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Тренировка таблицы умножения");
                    ui.separator();
                });

                //Main body
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), ui.available_height() - 30f32),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        
                        ui.add_space(ui.available_width() / 2f32 - 27f32);
                        ui.strong(self.first_num.to_string());
                        ui.label("*");
                        ui.strong(self.second_num.to_string());
                        ui.label("=");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.user_input).desired_width(14.6f32),
                        )
                        .request_focus();
                        
                        //Logic of the application
                        self.user_input = self.user_input.chars().take(2).collect::<String>();
                        match self.user_input.trim().parse::<u64>() {
                            Ok(user_answer) => {
                                if ctx.input().key_pressed(Key::Enter) {
                                    if user_answer == self.second_num * self.first_num {
                                        self.user_input = "".to_string();
                                        // First add
                                        self.combo += 1;
                                        // Then check
                                        if self.max_combo < self.combo {
                                            self.max_combo = self.combo;
                                        }
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

                        // Mybe it will be usefull
                        /*let mut window = egui::Window::new("title")
                            .id(egui::Id::new("demo_window_options")) // required since we change the title
                            .resizable(true)
                            .collapsible(true)
                            .title_bar(true)
                            .enabled(true);
                            window = window.anchor(egui::Align2::RIGHT_CENTER, vec2(0f32, 0f32));
                        window.show(ctx, |ui| ui.label("a"));*/

                    },
                );

                //Stats
                ui.group(|ui| {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.horizontal(|ui| {
                            
                            let max_combo = Label::new(format!("Максимальный счёт: {}", self.max_combo));
                            ui.add_sized([165f32,14f32], max_combo);

                            ui.add_space(ui.available_width() / 3f32 - 95f32);
                            
                            let sr_vremya = Label::new(format!("Ср. время: {}", self.average_time));
                            ui.add_sized([95f32,14f32], sr_vremya);
                            
                            ui.add_space(ui.available_width() / 2f32 - 85f32);
                            
                            let schyot = Label::new(format!("Счёт: {}", self.combo));
                            ui.add_sized([60f32,14f32], schyot);

                            ui.add_space(ui.available_width() - 110f32);                            

                            let popitka = Label::new(format!("Попытка № {}", self.try_count));
                            ui.add_sized([110f32,14f32], popitka);
                            
                        });
                    });
                });
            })
            .response
            .context_menu(|ui| {
                ui.menu_button("Тема", |ui| {
                    egui::global_dark_light_mode_buttons(ui);
                });
                ui.menu_button("Громкость", |ui| {
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0));
                    self.sl.set_global_volume(self.volume / 100f32);
                });
                ui.menu_button("О проекте", |ui| {
                    ui.horizontal(|ui| {
                        ui.weak("Made by");
                        ui.hyperlink_to("EvgenHi", "https://github.com/EvgenHi");
                    });
                });
                
                
            });
    }
}