use gtk4::prelude::*;
use gtk4::{TextBuffer, TextView, WrapMode};

pub struct Editor {
    pub view: TextView,
    pub buffer: TextBuffer,
}

impl Editor {
    pub fn new() -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::with_buffer(&buffer);
        view.set_hexpand(true);
        view.set_vexpand(true);
        view.set_wrap_mode(WrapMode::Word);

        Self { view, buffer }
    }

    pub fn get_content(&self) -> String {
        self.buffer
            .text(&self.buffer.start_iter(), &self.buffer.end_iter(), false)
            .to_string()
    }

    pub fn set_content(&self, content: &str) {
        self.buffer.set_text(content);
    }
}
