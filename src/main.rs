#![windows_subsystem = "windows"]

mod average_time;
mod theme;
use average_time::AverageTime;
use theme::CustomTheme;

use eframe::{
    egui::{self, style::Margin, FontData, FontDefinitions, Key, Label},
    epaint::{vec2, Color32, FontId},
    IconData,
};
use egui::epaint::FontFamily;
use rand::Rng;
use soloud::{AudioExt, LoadExt, Soloud, Wav};

fn main() {
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(715.0, 300.0)),
        max_window_size: None,
        initial_window_pos: Some(egui::pos2(548f32, 215f32)),
        initial_window_size: Some(egui::vec2(900 as f32, 600 as f32)),
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
    average_time: AverageTime,
    sl: Soloud,
    score_sound: Wav,
    ouch_sound: Wav,
    volume: f32,
    theme: CustomTheme,
    game_mode: GameMode,
    neverno_in_row: u64,
    stroke_color: Color32,
}

impl Default for MultiplicationTraining {
    fn default() -> Self {
        // Load sounds
        let mut sl = Soloud::default().unwrap();
        let mut score_sound = soloud::audio::Wav::default();
        score_sound
            .load_mem(include_bytes!("resources/score_point.wav"))
            .unwrap();
        let mut ouch_sound = soloud::audio::Wav::default();
        ouch_sound
            .load_mem(include_bytes!("resources/ouch.wav"))
            .unwrap();

        sl.set_global_volume(0.4f32);

        MultiplicationTraining {
            first_num: rand::thread_rng().gen_range(2..=9),
            second_num: rand::thread_rng().gen_range(2..=9),
            user_input: format!(""),
            combo: 0,
            max_combo: 0,
            try_count: 1,
            average_time: AverageTime::new(),
            sl,
            score_sound,
            ouch_sound,
            volume: 40f32,
            theme: CustomTheme::Dark,
            game_mode: GameMode::Arcade,
            neverno_in_row: 0,
            stroke_color: CustomTheme::Dark.stroke_standart(),
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

        use egui::FontFamily::Proportional;
        use egui::TextStyle::*;
        let mut style = (*cc.egui_ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(25.0, Proportional)),
            (Body, FontId::new(14.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();

        cc.egui_ctx.set_style(style);

        Self::default()
    }
}

impl eframe::App for MultiplicationTraining {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5f32);

        let central_panel_frame = egui::containers::Frame {
            inner_margin: Margin::symmetric(10f32, 10f32),
            outer_margin: Margin::symmetric(10f32, 10f32),
            rounding: egui::Rounding {
                nw: 10.0,
                ne: 10.0,
                sw: 10.0,
                se: 10.0,
            },
            fill: self.theme.bg_color(),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(central_panel_frame)
            .show(ctx, |ui| {
                ui.style_mut().visuals.override_text_color = Some(self.theme.text_color());

                // sets stroke color
                if self.average_time.return_to_standart_stroke() {
                    self.stroke_color = self.theme.stroke_standart();
                }
                ui.visuals_mut().selection.stroke.color = self.stroke_color;
                ui.visuals_mut().selection.stroke.width = 1.13f32;

                //Top heading
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Тренировка таблицы умножения");
                    ui.separator();
                });

                //Main body
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        ui.available_height()
                            - if self.game_mode == GameMode::Arcade {
                                30f32
                            } else {
                                0f32
                            },
                    ),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.style_mut().override_font_id = Some(FontId {
                            size: 20f32,
                            family: FontFamily::Proportional,
                        });
                        ui.add_space(ui.available_width() / 2f32 - 45f32); //27
                        ui.strong(self.first_num.to_string());
                        ui.label("*");
                        ui.strong(self.second_num.to_string());
                        ui.label("=");
                        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                            let mut layout_job: egui::text::LayoutJob =
                                egui::text::LayoutJob::simple_singleline(
                                    string.to_string(),
                                    FontId {
                                        size: 20f32,
                                        family: FontFamily::Proportional,
                                    },
                                    self.theme.text_color(),
                                );
                            layout_job.text = string.to_string();
                            ui.fonts().layout_job(layout_job)
                        };
                        let text_edit = egui::TextEdit::singleline(&mut self.user_input)
                            .desired_width(21f32) // 30
                            .layouter(&mut layouter);
                        ui.add(text_edit).request_focus();
                        if self.game_mode == GameMode::Training && self.neverno_in_row > 2 {
                            ui.weak(format!("// {}", self.first_num * self.second_num));
                        }

                        //Logic of the application
                        self.user_input = self.user_input.chars().take(2).collect::<String>();
                        match self.user_input.trim().parse::<u64>() {
                            Ok(user_answer) => {
                                if ctx.input().key_pressed(Key::Enter) {
                                    if user_answer == self.second_num * self.first_num {
                                        self.user_input = "".to_string();
                                        if self.game_mode == GameMode::Arcade {
                                            // First add
                                            self.combo += 1;
                                            // Then check
                                            if self.max_combo < self.combo {
                                                self.max_combo = self.combo;
                                            }
                                        } else {
                                            self.neverno_in_row = 0;
                                        }
                                        self.first_num = rand::thread_rng().gen_range(2..=9);
                                        self.second_num = rand::thread_rng().gen_range(2..=9);
                                        self.sl.play(&self.score_sound);
                                        self.average_time.count_again();
                                        self.stroke_color = self.theme.stroke_success();
                                    } else {
                                        self.user_input = "".to_string();
                                        if self.game_mode == GameMode::Arcade {
                                            if self.max_combo < self.combo {
                                                self.max_combo = self.combo;
                                            }
                                            self.combo = 0;
                                            self.try_count += 1;
                                        } else {
                                            self.neverno_in_row += 1;
                                        }
                                        self.sl.play(&self.ouch_sound);
                                        self.stroke_color = self.theme.stroke_failure();
                                        self.average_time = AverageTime::new();
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
                if self.game_mode == GameMode::Arcade {
                    ui.group(|ui| {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                            ui.horizontal(|ui| {
                                let max_combo =
                                    Label::new(format!("Максимальный счёт: {}", self.max_combo));
                                ui.add_sized([165f32, 14f32], max_combo);

                                ui.add_space(ui.available_width() / 3f32 - 95f32);

                                let sr_vremya = Label::new(format!(
                                    "Ср. время: {}",
                                    self.average_time.get_overall_time()
                                ));
                                ui.add_sized([95f32, 14f32], sr_vremya);

                                ui.add_space(ui.available_width() / 2f32 - 85f32);

                                let schyot = Label::new(format!("Счёт: {}", self.combo));
                                ui.add_sized([60f32, 14f32], schyot);

                                ui.add_space(ui.available_width() - 110f32);

                                let popitka = Label::new(format!("Попытка #{}", self.try_count));
                                ui.add_sized([110f32, 14f32], popitka);
                            });
                        });
                    });
                }
            })
            .response
            .context_menu(|ui| {
                ui.menu_button("Режим", |ui| {
                    if ui
                        .selectable_value(&mut self.game_mode, GameMode::Arcade, "Аркадный")
                        .changed()
                    {
                        self.average_time = AverageTime::new();
                    };
                    ui.selectable_value(&mut self.game_mode, GameMode::Training, "Тренировочный");
                });

                ui.menu_button("Тема", |ui| {
                    ui.selectable_value(&mut self.theme, CustomTheme::Dark, "Тёмная");
                    ui.selectable_value(&mut self.theme, CustomTheme::Light, "Светлая");
                    ui.selectable_value(&mut self.theme, CustomTheme::Red, "Красная");
                    ui.selectable_value(&mut self.theme, CustomTheme::Green, "Зелёная");
                    ui.selectable_value(&mut self.theme, CustomTheme::Blue, "Голубая");
                    ui.selectable_value(&mut self.theme, CustomTheme::Transparent, "Прозрачная");
                    ui.selectable_value(&mut self.theme, CustomTheme::WindowsXP, "WindowsXP");
                });

                ui.menu_button("Громкость", |ui| {
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0));
                    self.sl.set_global_volume(self.volume / 100f32);
                });

                ui.menu_button("О проекте", |ui| {
                    ui.vertical(|ui| {
                        ui.label(r"\/\/\ v0.4.0 /\/\/");
                        ui.horizontal(|ui| {
                            ui.add_space(11f32);
                            ui.weak("Made by");
                            ui.hyperlink_to("EvgenHi", "https://github.com/EvgenHi");
                        });
                        ui.label(r"\/\/\/\/\/\/\/\/\/");
                    });
                });
            });
        ctx.request_repaint();
    }
}

#[derive(PartialEq)]
enum GameMode {
    Training,
    Arcade,
}
