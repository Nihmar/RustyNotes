use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use tracing::info;

pub struct App {
    pub gtk_app: Application,
    pub window: ApplicationWindow,
}

impl App {
    pub fn new() -> Self {
        let gtk_app = Application::new(Some("com.rustynotes.app"), ApplicationFlags::default());

        let window = ApplicationWindow::new(&gtk_app);
        window.set_default_size(1200, 800);
        window.set_title(Some("RustyNotes"));

        gtk_app.connect_activate(move |app| {
            info!("RustyNotes activated");
            let window = ApplicationWindow::new(app);
            window.set_default_size(1200, 800);
            window.set_title(Some("RustyNotes"));
            window.present();
        });

        Self { gtk_app, window }
    }

    pub fn run(&self) {
        self.gtk_app.run();
    }
}
