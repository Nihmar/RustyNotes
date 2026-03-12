use crate::models::note::Note;
use crate::storage::file;
use crate::ui::editor::Editor;
use crate::ui::preview::Preview;
use crate::ui::sidebar::Sidebar;
use crate::ui::window::MainWindow;
use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, HeaderBar, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    pub gtk_app: Application,
}

impl App {
    pub fn new() -> Self {
        let gtk_app = Application::new(Some("com.rustynotes.app"), ApplicationFlags::default());

        gtk_app.connect_activate(|app| {
            let window = ApplicationWindow::new(app);
            window.set_default_size(1200, 800);
            window.set_title(Some("RustyNotes"));

            let headerbar = HeaderBar::new();
            window.set_titlebar(Some(&headerbar));

            let main_window = MainWindow::new();
            let sidebar = Sidebar::new();
            let editor = Editor::new();
            let preview = Preview::new();
            let current_note: Rc<RefCell<Option<Note>>> = Rc::new(RefCell::new(None));

            let editor_scrolled = ScrolledWindow::new();
            editor_scrolled.set_hexpand(true);
            editor_scrolled.set_vexpand(true);
            editor_scrolled.set_child(Some(&editor.view));

            let preview_scrolled = ScrolledWindow::new();
            preview_scrolled.set_hexpand(true);
            preview_scrolled.set_vexpand(true);
            preview_scrolled.set_child(Some(&preview.view));
            preview_scrolled.set_visible(false);

            let sidebar_clone = sidebar.clone();
            let editor_clone = editor.clone();
            let preview_clone = preview.clone();
            let current_note_clone = current_note.clone();

            let open_button = Button::with_label("Open Folder");
            let save_button = Button::with_label("Save");
            let edit_button = Button::with_label("Edit");
            let preview_button = Button::with_label("Preview");

            let window_clone = window.clone();
            open_button.connect_clicked(move |_| {
                use gtk4::FileChooserAction;
                use gtk4::FileChooserNative;

                let folder_chooser = FileChooserNative::new(
                    Some("Open Folder"),
                    Some(&window_clone),
                    FileChooserAction::SelectFolder,
                    Some("Open"),
                    Some("Cancel"),
                );

                let sidebar_c = sidebar_clone.clone();
                let editor_c = editor_clone.clone();
                let preview_c = preview_clone.clone();
                let current_note_c = current_note_clone.clone();

                folder_chooser.connect_response(move |chooser, response| {
                    if response == gtk4::ResponseType::Accept {
                        if let Some(file) = chooser.file() {
                            if let Some(path) = file.path() {
                                tracing::info!("Opening folder: {:?}", path);

                                let editor_for_cb = editor_c.clone();
                                let preview_for_cb = preview_c.clone();
                                let current_note_for_cb = current_note_c.clone();

                                sidebar_c.load_folder(path, move |note: Note| {
                                    tracing::info!("Opening note: {}", note.title);
                                    if let Ok(content) = file::read_file(&note.path) {
                                        editor_for_cb.set_content(&content);
                                        preview_for_cb.render(&content);
                                        *current_note_for_cb.borrow_mut() = Some(note);
                                    }
                                });
                            }
                        }
                    }
                });

                folder_chooser.show();
            });

            let editor_for_save = editor.clone();
            let current_note_for_save = current_note.clone();
            save_button.connect_clicked(move |_| {
                if let Some(ref note) = *current_note_for_save.borrow() {
                    let content = editor_for_save.get_content();
                    if let Err(e) = file::write_file(&note.path, &content) {
                        tracing::error!("Failed to save note: {}", e);
                    } else {
                        tracing::info!("Note saved: {}", note.title);
                    }
                }
            });

            let editor_scrolled_for_toggle = editor_scrolled.clone();
            let preview_scrolled_for_toggle = preview_scrolled.clone();
            edit_button.connect_clicked(move |_| {
                editor_scrolled_for_toggle.set_visible(true);
                preview_scrolled_for_toggle.set_visible(false);
            });

            let editor_for_preview = editor.clone();
            let preview_for_preview = preview.clone();
            let editor_scrolled_for_preview = editor_scrolled.clone();
            let preview_scrolled_for_preview = preview_scrolled.clone();
            preview_button.connect_clicked(move |_| {
                let markdown = editor_for_preview.get_content();
                preview_for_preview.render(&markdown);
                editor_scrolled_for_preview.set_visible(false);
                preview_scrolled_for_preview.set_visible(true);
            });

            headerbar.pack_start(&open_button);
            headerbar.pack_start(&edit_button);
            headerbar.pack_start(&preview_button);
            headerbar.pack_end(&save_button);

            main_window.sidebar.append(&sidebar.container);
            main_window.content.append(&editor_scrolled);
            main_window.content.append(&preview_scrolled);
            window.set_child(Some(&main_window.paned));

            window.present();
        });

        Self { gtk_app }
    }

    pub fn run(&self) {
        self.gtk_app.run();
    }
}
