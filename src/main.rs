mod app;
mod vault;
mod editor;
mod graph;
mod search;
mod theme;
mod config;

fn theme_fn(_state: &app::Model) -> iced::Theme {
    theme::dark_theme()
}

fn main() -> iced::Result {
    let recent = config::load_recent_vaults();
    iced::application(move || app::new(recent.clone()), app::update, app::view)
        .title("RustyNotes")
        .theme(theme_fn)
        .window(iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            ..Default::default()
        })
        .run()
}
