use gtk4::prelude::*;
use gtk4::{TextBuffer, TextTag, TextTagTable, TextView, WrapMode};
use regex::Regex;
use std::cell::RefCell;
use std::sync::{Arc, OnceLock};

struct RegexPatterns {
    h1: Regex,
    h2: Regex,
    h3: Regex,
    bold: Regex,
    italic: Regex,
    code_inline: Regex,
    code_block: Regex,
    link: Regex,
    list: Regex,
}

impl RegexPatterns {
    fn new() -> Self {
        Self {
            h1: Regex::new(r"^# .+$").unwrap(),
            h2: Regex::new(r"^## .+$").unwrap(),
            h3: Regex::new(r"^### .+$").unwrap(),
            bold: Regex::new(r"\*\*[^*]+\*\*|__[^_]+__").unwrap(),
            italic: Regex::new(r"\*[^*]+\*|_[^_]+_").unwrap(),
            code_inline: Regex::new(r"`[^`]+`").unwrap(),
            code_block: Regex::new(r"```[\s\S]*?```").unwrap(),
            link: Regex::new(r"\[[^\]]+\]\([^)]+\)").unwrap(),
            list: Regex::new(r"^[\s]*[-*+]\s").unwrap(),
        }
    }
}

static REGEX_PATTERNS: OnceLock<RegexPatterns> = OnceLock::new();

fn get_regex_patterns() -> &'static RegexPatterns {
    REGEX_PATTERNS.get_or_init(RegexPatterns::new)
}

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
        let header1 = TextTag::new(Some("h1"));
        header1.set_property("size-points", 24.0);
        header1.set_property("weight", 700);
        header1.set_property("foreground", &"#2563eb");
        self.tag_table.add(&header1);

        let header2 = TextTag::new(Some("h2"));
        header2.set_property("size-points", 20.0);
        header2.set_property("weight", 600);
        header2.set_property("foreground", &"#1d4ed8");
        self.tag_table.add(&header2);

        let header3 = TextTag::new(Some("h3"));
        header3.set_property("size-points", 18.0);
        header3.set_property("weight", 600);
        header3.set_property("foreground", &"#1e40af");
        self.tag_table.add(&header3);

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

        let list = TextTag::new(Some("list"));
        list.set_property("foreground", &"#059669");
        self.tag_table.add(&list);
    }

    pub fn get_buffer(&self) -> TextBuffer {
        self.buffer.clone()
    }

    pub fn apply_highlighting(&self, text: &str) {
        self.buffer.set_text(text);

        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.remove_all_tags(&start, &end);

        let patterns = get_regex_patterns();

        self.apply_regex("h1", &patterns.h1, text);
        self.apply_regex("h2", &patterns.h2, text);
        self.apply_regex("h3", &patterns.h3, text);
        self.apply_regex("bold", &patterns.bold, text);
        self.apply_regex("italic", &patterns.italic, text);
        self.apply_regex("code", &patterns.code_inline, text);
        self.apply_regex("code", &patterns.code_block, text);
        self.apply_regex("link", &patterns.link, text);
        self.apply_regex("list", &patterns.list, text);
    }

    fn apply_regex(&self, tag_name: &str, regex: &Regex, text: &str) {
        if let Some(tag) = self.tag_table.lookup(tag_name) {
            for mat in regex.find_iter(text) {
                let mut start = self.buffer.start_iter();
                start.set_offset(mat.start() as i32);
                let mut end = self.buffer.start_iter();
                end.set_offset(mat.end() as i32);
                self.buffer.apply_tag(&tag, &start, &end);
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
        view.set_wrap_mode(WrapMode::Word);
        view.set_monospace(true);
        view.set_accepts_tab(true);
        view.set_indent(2);

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
