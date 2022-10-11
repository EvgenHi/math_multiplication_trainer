use eframe::epaint::Color32;

#[derive(PartialEq, Clone, Copy)]
pub enum CustomTheme {
    Dark,
    Light,
    Red,
    Blue,
    Green,
    Transparent,
    WindowsXP,
}

impl CustomTheme {
    pub fn bg_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(27, 27, 27, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(248, 248, 248, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(100, 216, 211, 235),
            CustomTheme::Red => Color32::from_rgba_premultiplied(189, 46, 54, 235),
            CustomTheme::Green => Color32::from_rgba_premultiplied(106, 232, 110, 235),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(0, 0, 0, 0),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(212, 208, 200, 255),
        }
    }

    pub fn text_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(150, 150, 150, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(0, 0, 0, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(0, 0, 0, 255),
            CustomTheme::Red => Color32::from_rgba_premultiplied(0, 0, 0, 255),
            CustomTheme::Green => Color32::from_rgba_premultiplied(0, 0, 0, 255),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(215, 215, 215, 255),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(0, 0, 0, 255),
        }
    }

    // "stroke" is these line around user_input text box
    pub fn stroke_standart_color(&self) -> Color32 {
        match self {
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(61, 149, 255, 255),
            _ => Color32::from_rgba_premultiplied(193, 223, 255, 255),
        }
    }

    pub fn stroke_success_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(46, 204, 113, 255),
        }
    }

    pub fn stroke_failure_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(217, 30, 24, 255),
        }
    }

    pub fn user_input_bg_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(0, 0, 0, 255),
        }
    }

    pub fn eye_catching_text_color(&self) -> Color32 {
        match self {
            CustomTheme::Light => Color32::from_rgba_premultiplied(0, 0, 0, 255),
            _ => Color32::from_rgba_premultiplied(255, 255, 255, 255),
        }
    }

    pub fn heading_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(160, 160, 160, 255),
        }
    }

    pub fn window_stroke_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(0, 0, 0, 0),
        }
    }

    pub fn top_separator_color(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(160, 160, 160, 255),
        }
    }

    pub fn top_separator_width(&self) -> f32 {
        match self {
            _ => 1f32,
        }
    }

    // theme_button function family only change text color of corresponding theme button (in other words regardless to current theme)
    pub fn theme_button_bg_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(15, 15, 15, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(200, 200, 200, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(100, 216, 211, 235),
            CustomTheme::Red => Color32::from_rgba_premultiplied(189, 46, 54, 235),
            CustomTheme::Green => Color32::from_rgba_premultiplied(106, 232, 110, 235),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(80, 80, 80, 1),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(59, 119, 188, 235),
        }
    }

    pub fn theme_button_text_active_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(230, 230, 230, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(40, 40, 40, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(41, 153, 134, 255),
            CustomTheme::Red => Color32::from_rgba_premultiplied(97, 10, 10, 255),
            CustomTheme::Green => Color32::from_rgba_premultiplied(22, 99, 6, 255),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(180, 180, 180, 255),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(33, 27, 150, 255),
        }
    }

    pub fn theme_button_text_inactive_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(15, 15, 15, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(200, 200, 200, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(100, 216, 211, 235),
            CustomTheme::Red => Color32::from_rgba_premultiplied(189, 46, 54, 235),
            CustomTheme::Green => Color32::from_rgba_premultiplied(106, 232, 110, 235),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(80, 80, 80, 1),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(59, 119, 188, 235),
        }
    }
  
    
}
