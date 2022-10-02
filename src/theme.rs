use eframe::epaint::Color32;

pub enum CustomTheme {
    Dark,
    Light,
    Blue,
    Red,
    Green,
    WindowsXP,
    Transparent,
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
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(59, 119, 188, 235),
        }
    }

    pub fn text_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(150, 150, 150, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(105, 105, 105, 255),
            CustomTheme::Blue => Color32::from_rgba_premultiplied(100, 100, 100, 255),
            CustomTheme::Red => Color32::from_rgba_premultiplied(150, 150, 150, 255),
            CustomTheme::Green => Color32::from_rgba_premultiplied(100, 100, 100, 255),
            CustomTheme::Transparent => Color32::from_rgba_premultiplied(180, 180, 180, 255),
            CustomTheme::WindowsXP => Color32::from_rgba_premultiplied(129, 192, 70, 235),
        }
    }
}
