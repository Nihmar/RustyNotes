use iced::theme::Palette;
use iced::Color;

pub fn dark_palette() -> Palette {
    Palette {
        background: Color::from_rgb(0.13, 0.13, 0.14),
        text: Color::from_rgb(0.85, 0.85, 0.85),
        primary: Color::from_rgb(0.55, 0.35, 0.85),
        success: Color::from_rgb(0.35, 0.75, 0.45),
        warning: Color::from_rgb(0.85, 0.65, 0.25),
        danger: Color::from_rgb(0.85, 0.35, 0.35),
    }
}

pub fn dark_theme() -> iced::Theme {
    iced::Theme::custom("RustyNotes Dark".to_string(), dark_palette())
}
