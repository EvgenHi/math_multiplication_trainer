use eframe::epaint::Color32;

#[derive(PartialEq)]
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
    pub fn stroke_standart(&self) -> Color32 {
        match self {
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(61, 149, 255, 255),
            _ => Color32::from_rgba_premultiplied(193, 223, 255, 255),
        }
    }

    pub fn stroke_success(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(46, 204, 113, 255),
        }
    }

    pub fn stroke_failure(&self) -> Color32 {
        match self {
            _ => Color32::from_rgba_premultiplied(217, 30, 24, 255),
        }
    }
}
