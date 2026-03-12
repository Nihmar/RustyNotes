use gtk4::prelude::*;
use gtk4::{Box, Orientation, Paned};

pub struct MainWindow {
    pub paned: Paned,
    pub sidebar: Box,
    pub content: Box,
}

impl MainWindow {
    pub fn new() -> Self {
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_position(220);

        let sidebar = Box::new(Orientation::Vertical, 0);
        sidebar.set_hexpand(true);
        sidebar.set_vexpand(true);

        let content = Box::new(Orientation::Vertical, 0);
        content.set_hexpand(true);
        content.set_vexpand(true);

        paned.set_start_child(Some(&sidebar));
        paned.set_end_child(Some(&content));

        Self {
            paned,
            sidebar,
            content,
        }
    }
}
