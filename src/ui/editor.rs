use gtk4::prelude::*;
use gtk4::{TextBuffer, TextTag, TextTagTable, TextView, WrapMode};
use std::cell::RefCell;
use std::sync::Arc;

pub struct MarkdownHighlighter {
    buffer: TextBuffer,
    tag_table: TextTagTable,
}

impl MarkdownHighlighter {
    pub fn new() -> Self {
        let tag_table = TextTagTable::new();
        let buffer = TextBuffer::new(Some(&tag_table));

        let highlighter = Self { buffer, tag_table };
        highlighter.setup_tags();
        highlighter
    }

    fn setup_tags(&self) {
        let h1 = TextTag::new(Some("h1"));
        h1.set_property("size-points", 24.0);
        h1.set_property("weight", 700);
        h1.set_property("foreground", &"#2563eb");
        self.tag_table.add(&h1);

        let h2 = TextTag::new(Some("h2"));
        h2.set_property("size-points", 20.0);
        h2.set_property("weight", 600);
        h2.set_property("foreground", &"#1d4ed8");
        self.tag_table.add(&h2);

        let h3 = TextTag::new(Some("h3"));
        h3.set_property("size-points", 18.0);
        h3.set_property("weight", 600);
        h3.set_property("foreground", &"#1e40af");
        self.tag_table.add(&h3);

        let tag = TextTag::new(Some("tag"));
        tag.set_property("foreground", &"#dc2626");
        self.tag_table.add(&tag);

        let checkbox = TextTag::new(Some("checkbox"));
        checkbox.set_property("foreground", &"#7c3aed");
        checkbox.set_property("weight", 600);
        self.tag_table.add(&checkbox);

        let list_marker = TextTag::new(Some("list"));
        list_marker.set_property("foreground", &"#059669");
        self.tag_table.add(&list_marker);

        let bold = TextTag::new(Some("bold"));
        bold.set_property("weight", 700);
        self.tag_table.add(&bold);

        let italic = TextTag::new(Some("italic"));
        italic.set_property("style", gtk4::pango::Style::Italic);
        self.tag_table.add(&italic);

        let code = TextTag::new(Some("code"));
        code.set_property("family", &"monospace");
        code.set_property("background", &"#f1f5f9");
        code.set_property("foreground", &"#0f172a");
        self.tag_table.add(&code);

        let link = TextTag::new(Some("link"));
        link.set_property("foreground", &"#0369a1");
        link.set_property("underline", gtk4::pango::Underline::Single);
        self.tag_table.add(&link);
    }

    pub fn get_buffer(&self) -> TextBuffer {
        self.buffer.clone()
    }

    pub fn apply_highlighting(&self, text: &str) {
        self.buffer.set_text(text);

        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.remove_all_tags(&start, &end);

        let total_len = end.offset();

        let mut pos: i32 = 0;
        let mut in_code_block = false;

        for line in text.lines() {
            let line_len = line.len() as i32;

            if line.is_empty() {
                pos += 1;
                continue;
            }

            if !in_code_block && line.starts_with("```") {
                in_code_block = true;
                self.apply_tag("code", pos, pos + line_len, total_len);
                pos += line_len + 1;
                continue;
            }

            if in_code_block && line.starts_with("```") {
                in_code_block = false;
                self.apply_tag("code", pos, pos + line_len, total_len);
                pos += line_len + 1;
                continue;
            }

            if in_code_block {
                self.apply_tag("code", pos, pos + line_len, total_len);
                pos += line_len + 1;
                continue;
            }

            let trimmed = line.trim_start();

            if trimmed.starts_with("### ") {
                self.apply_tag(
                    "h3",
                    pos + (line.len() as i32 - trimmed.len() as i32),
                    pos + line_len,
                    total_len,
                );
                pos += line_len + 1;
                continue;
            }
            if trimmed.starts_with("## ") {
                self.apply_tag(
                    "h2",
                    pos + (line.len() as i32 - trimmed.len() as i32),
                    pos + line_len,
                    total_len,
                );
                pos += line_len + 1;
                continue;
            }
            if trimmed.starts_with("# ") {
                self.apply_tag(
                    "h1",
                    pos + (line.len() as i32 - trimmed.len() as i32),
                    pos + line_len,
                    total_len,
                );
                pos += line_len + 1;
                continue;
            }

            if trimmed.starts_with("- [")
                || trimmed.starts_with("* [")
                || trimmed.starts_with("+ [")
            {
                let marker_end = if trimmed.starts_with("- [") {
                    4
                } else if trimmed.starts_with("* [") {
                    4
                } else {
                    4
                };
                self.apply_tag("checkbox", pos, pos + marker_end, total_len);
                pos += line_len + 1;
                continue;
            }

            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
                self.apply_tag("list", pos, pos + 2, total_len);
                pos += line_len + 1;
                continue;
            }

            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'`' {
                    let mut j = i + 1;
                    while j < bytes.len() && bytes[j] != b'`' {
                        j += 1;
                    }
                    if j > i + 1 && j < bytes.len() {
                        self.apply_tag("code", pos + i as i32, pos + j as i32 + 1, total_len);
                        i = j + 1;
                        continue;
                    }
                }
                if bytes[i] == b'[' {
                    let mut j = i + 1;
                    while j < bytes.len() && bytes[j] != b']' {
                        j += 1;
                    }
                    if j + 1 < bytes.len() && bytes[j + 1] == b'(' {
                        let mut k = j + 2;
                        while k < bytes.len() && bytes[k] != b')' {
                            k += 1;
                        }
                        if k < bytes.len() {
                            self.apply_tag("link", pos + i as i32, pos + k as i32 + 1, total_len);
                            i = k + 1;
                            continue;
                        }
                    }
                }
                if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'*' {
                    let mut j = i + 2;
                    while j + 1 < bytes.len() && (bytes[j] != b'*' || bytes[j + 1] != b'*') {
                        j += 1;
                    }
                    if j + 1 < bytes.len() {
                        self.apply_tag("bold", pos + i as i32, pos + j as i32 + 2, total_len);
                        i = j + 2;
                        continue;
                    }
                }
                if bytes[i] == b'*' && (i == 0 || bytes[i - 1] != b'*') {
                    let mut j = i + 1;
                    while j < bytes.len() && bytes[j] != b'*' && bytes[j] != b' ' {
                        j += 1;
                    }
                    if j < bytes.len() && bytes[j] == b'*' {
                        self.apply_tag("italic", pos + i as i32, pos + j as i32 + 1, total_len);
                        i = j + 1;
                        continue;
                    }
                }
                i += 1;
            }

            let line_start = pos + (line.len() as i32 - trimmed.len() as i32);
            if let Some(tag_idx) = trimmed.find('#') {
                if tag_idx + 1 < trimmed.len() {
                    let c = trimmed.chars().nth(tag_idx + 1).unwrap();
                    if c.is_alphabetic() {
                        self.apply_tag(
                            "tag",
                            line_start + tag_idx as i32,
                            line_start + trimmed.len() as i32,
                            total_len,
                        );
                    }
                }
            }

            pos += line_len + 1;
        }
    }

    fn apply_tag(&self, tag_name: &str, start: i32, end: i32, total_len: i32) {
        if start >= 0 && start < total_len && end > start && end <= total_len {
            if let Some(tag) = self.tag_table.lookup(tag_name) {
                let mut start_iter = self.buffer.start_iter();
                start_iter.set_offset(start);
                let mut end_iter = self.buffer.start_iter();
                end_iter.set_offset(end);
                self.buffer.apply_tag(&tag, &start_iter, &end_iter);
            }
        }
    }
}

#[derive(Clone)]
pub struct Editor {
    pub view: TextView,
    pub buffer: TextBuffer,
    highlighter: Arc<RefCell<MarkdownHighlighter>>,
}

impl Editor {
    pub fn new() -> Self {
        let highlighter = Arc::new(RefCell::new(MarkdownHighlighter::new()));
        let buffer = highlighter.borrow().get_buffer();

        let view = TextView::with_buffer(&buffer);
        view.set_hexpand(true);
        view.set_vexpand(true);
        view.set_wrap_mode(WrapMode::WordChar);
        view.set_monospace(true);
        view.set_accepts_tab(true);
        view.set_indent(2);
        view.set_left_margin(8);
        view.set_right_margin(8);

        Self {
            view,
            buffer,
            highlighter,
        }
    }

    pub fn get_content(&self) -> String {
        self.buffer
            .text(&self.buffer.start_iter(), &self.buffer.end_iter(), false)
            .to_string()
    }

    pub fn set_content(&self, content: &str) {
        self.highlighter.borrow().apply_highlighting(content);
    }
}
