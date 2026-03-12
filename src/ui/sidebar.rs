use crate::models::note::Note;
use crate::storage::folder::FolderManager;
use gtk4::prelude::*;
use gtk4::{CellRendererText, ListStore, ScrolledWindow, TreeView, TreeViewColumn};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Sidebar {
    pub container: ScrolledWindow,
    pub tree_view: TreeView,
    pub list_store: ListStore,
    pub current_folder: Rc<RefCell<Option<PathBuf>>>,
    notes: Rc<RefCell<Vec<Note>>>,
}

impl Sidebar {
    pub fn new() -> Self {
        let list_store = ListStore::new(&[glib::Type::STRING, glib::Type::STRING]);

        let tree_view = TreeView::new();
        tree_view.set_hexpand(true);
        tree_view.set_vexpand(true);
        tree_view.set_headers_visible(false);

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        tree_view.append_column(&column);

        tree_view.set_model(Some(&list_store));

        let container = ScrolledWindow::new();
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_child(Some(&tree_view));

        Self {
            container,
            tree_view,
            list_store,
            current_folder: Rc::new(RefCell::new(None)),
            notes: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn load_folder<F>(&self, path: PathBuf, on_select: F)
    where
        F: Fn(Note) + Clone + 'static,
    {
        *self.current_folder.borrow_mut() = Some(path.clone());

        let folder = match FolderManager::new(path) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to open folder: {}", e);
                return;
            }
        };

        let notes = match folder.scan_notes() {
            Ok(n) => n,
            Err(e) => {
                tracing::error!("Failed to scan notes: {}", e);
                return;
            }
        };

        *self.notes.borrow_mut() = notes.clone();
        self.list_store.clear();

        let root_path = folder.root.clone();
        let root_name = root_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Notes".to_string());

        let root_iter = self.list_store.append();
        self.list_store
            .set_value(&root_iter, 0, &format!("[{}]", root_name).to_value());
        self.list_store
            .set_value(&root_iter, 1, &String::new().to_value());

        let mut folders: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

        for note in &notes {
            let parent_path = note.path.parent().unwrap_or(&root_path);
            let relative = parent_path.strip_prefix(&root_path).unwrap_or(parent_path);

            for component in relative.iter() {
                let comp_path = root_path.join(component);
                if folders.insert(comp_path.clone()) {
                    let folder_iter = self.list_store.append();
                    let depth = relative.iter().take_while(|p| **p != *component).count();
                    let prefix = "  ".repeat(depth);
                    self.list_store.set_value(
                        &folder_iter,
                        0,
                        &format!("{}📁 {}", prefix, component.to_string_lossy()).to_value(),
                    );
                    self.list_store
                        .set_value(&folder_iter, 1, &String::new().to_value());
                }
            }

            let depth = relative.iter().count();
            let prefix = "  ".repeat(depth);
            let iter = self.list_store.append();
            self.list_store
                .set_value(&iter, 0, &format!("{}📄 {}", prefix, note.title).to_value());
            self.list_store
                .set_value(&iter, 1, &note.path.to_string_lossy().to_value());
        }

        let notes_clone = notes.clone();
        let on_select_clone = on_select.clone();
        let list_store_clone = self.list_store.clone();
        self.tree_view
            .connect_row_activated(move |_tree_view, path, _| {
                if let Some(iter) = list_store_clone.iter(path) {
                    let path_str: String = list_store_clone.get_value(&iter, 1).get().unwrap();
                    if !path_str.is_empty() {
                        let path = PathBuf::from(path_str);

                        if let Some(note) = notes_clone.iter().find(|n| n.path == path) {
                            on_select_clone(note.clone());
                        }
                    }
                }
            });
    }
}

impl Clone for Sidebar {
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
            tree_view: self.tree_view.clone(),
            list_store: self.list_store.clone(),
            current_folder: self.current_folder.clone(),
            notes: self.notes.clone(),
        }
    }
}
