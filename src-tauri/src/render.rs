use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

fn preprocess_images(input: &str) -> String {
    let re = Regex::new(r"!\[\[([^\]]+?)(?:\|([^\]]+?))?\]\]").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let target = caps.get(1).unwrap().as_str();
        let alt = caps.get(2).map_or(target, |m| m.as_str());
        format!(
            r#"<img class="image-embed" src="vault://localhost/{}" alt="{}" loading="lazy">"#,
            target, alt
        )
    })
    .to_string()
}

fn preprocess_wikilinks(input: &str) -> String {
    let re = Regex::new(r"\[\[([^\]]+?)(?:\|([^\]]+?))?\]\]").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let target = caps.get(1).unwrap().as_str();
        let display = caps.get(2).map_or(target, |m| m.as_str());
        format!(
            r#"<a class="wikilink" href="note://{}">{}</a>"#,
            target, display
        )
    })
    .to_string()
}

fn preprocess_math(input: &str) -> String {
    let re_block = Regex::new(r"(?s)\$\$\n?(.*?)\n?\$\$").unwrap();
    let after_block = re_block.replace_all(input, |caps: &regex::Captures| {
        let content = caps.get(1).unwrap().as_str().trim();
        format!(r#"<div class="math-block">{}</div>"#, content)
    });

    let re_inline = Regex::new(r"\$([^$\n]+?)\$").unwrap();
    let bytes = after_block.as_bytes();
    let mut result = String::with_capacity(after_block.len());
    let mut last_end = 0;

    for caps in re_inline.captures_iter(&after_block) {
        let m = caps.get(0).unwrap();
        let start = m.start();
        let end = m.end();

        if start > 0 && bytes.get(start.wrapping_sub(1)) == Some(&b'$') {
            continue;
        }
        if bytes.get(end) == Some(&b'$') {
            continue;
        }

        result.push_str(&after_block[last_end..start]);
        let content = caps.get(1).unwrap().as_str().trim();
        result.push_str(&format!(
            r#"<span class="math-inline">{}</span>"#,
            content
        ));
        last_end = end;
    }
    result.push_str(&after_block[last_end..]);
    result
}

#[tauri::command]
pub fn render_markdown(content: String) -> Result<String, String> {
    Ok(render_markdown_inner(&content))
}

fn render_markdown_inner(content: &str) -> String {
    let content = preprocess_images(content);
    let content = preprocess_wikilinks(&content);
    let content = preprocess_math(&content);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
