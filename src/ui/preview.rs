use gtk4::prelude::*;
use gtk4::{TextBuffer, TextView, WrapMode};
use pulldown_cmark::{html, Options, Parser};

pub struct Preview {
    pub view: TextView,
    buffer: TextBuffer,
}

impl Preview {
    pub fn new() -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::with_buffer(&buffer);
        view.set_hexpand(true);
        view.set_vexpand(true);
        view.set_editable(false);
        view.set_wrap_mode(WrapMode::Word);

        Self { view, buffer }
    }

    pub fn render(&self, markdown: &str) {
        let parser = Parser::new_ext(markdown, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        self.buffer.set_text(&html_output);
    }
}
