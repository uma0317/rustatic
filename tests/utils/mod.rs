use rustatic::utils;
use tera::Context;

#[test]
fn it_convert_md_to_html() {
    assert_eq!(utils::md_to_html("Hello, **世界**!"),
           "<p>Hello, <strong>世界</strong>!</p>\n");
}

#[test]
fn it_count_md_files() {
    assert_eq!(utils::count_md_files().unwrap(), 3);
}

// #[test]
// fn html_generate_test() {
//     utils::create_html_file("test", "hi");
// }

#[test]
fn htmls_generate_test() {
    utils::generate_html_files();
}

#[test]
fn render_test() {
    let mut context = Context::new();
    context.add("title", &"test");
    context.add("contents", &"testcontent");
    match utils::render(context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{}", e)
    };
}


#[test]
fn get_all_articles_test() {
    println!("{:?}", utils::get_all_articles());
    println!("{:?}", utils::get_all_articles().unwrap().len());
}

#[test]
fn create_articles_list_test() {
    println!("{:?}", utils::create_articles_list());
}

#[test]
fn generate_index_html_test() {
    println!("{:?}", utils::generate_index_html());
}