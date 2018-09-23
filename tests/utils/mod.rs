use rustatic::utils;

#[test]
fn it_convert_md_to_html() {
    assert_eq!(utils::md_to_html("Hello, **世界**!"),
           "<p>Hello, <strong>世界</strong>!</p>\n");
}

#[test]
fn it_count_md_files() {
    assert_eq!(utils::count_md_files().unwrap(), 3);
}