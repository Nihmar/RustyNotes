use gtk4::prelude::*;
use gtk4::{Box, ColumnView, Orientation, SingleSelection, StringList};

pub struct Sidebar {
    pub container: Box,
    pub note_list: ColumnView,
    pub selection: SingleSelection,
    pub string_list: StringList,
}

impl Sidebar {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.set_hexpand(true);

        let string_list = StringList::new(&[]);
        let selection = SingleSelection::new(Some(string_list.clone()));
        let note_list = ColumnView::new(Some(selection.clone()));
        note_list.set_hexpand(true);
        note_list.set_vexpand(true);

        container.append(&note_list);

        Self {
            container,
            note_list,
            selection,
            string_list,
        }
    }
}
