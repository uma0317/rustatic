use comrak::{markdown_to_html, ComrakOptions};

pub fn md_to_html(md: &str) -> String {
    markdown_to_html(md, &ComrakOptions::default())
}