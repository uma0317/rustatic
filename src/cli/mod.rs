use clap::{App, AppSettings, Arg, SubCommand, ArgMatches};
use chrono::prelude::*;

use std::fs;
use std::io;
use std::io::{ Write, BufReader };
use std::io::prelude::*;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use utils;
use statics::{ SOURCE_PATH, SOURCE_POSTS_PATH, SOURCE_LAYOUTS_PATH, DIST_POSTS_PATH };

pub fn init() {
    let matches = build().get_matches();
    arg_init(&matches);
    arg_new(&matches);
    arg_generate(&matches);
}

pub fn build() -> App<'static, 'static> {
    App::new("clapex")
        .version("0.1.0")                       
        .author("myname <myname@mail.com>")     
        .about("Clap Example CLI")              
        .subcommand(SubCommand::with_name("init")
            .about("init new blog")
            .arg(Arg::with_name("Blog name")
                .help("Blog name")
                .required(true)
                .takes_value(true) 
            )
        )
        .subcommand(SubCommand::with_name("new")
            .about("create new article")
            .arg(Arg::with_name("Article title")
                .help("Article title")
                .required(true)
                .takes_value(true) 
            )
        )
        .subcommand(SubCommand::with_name("generate")
            .about("generate static files")
        )

        .subcommand(SubCommand::with_name("add")
            .about("add folder to list")
            .arg(Arg::with_name("Target Project path")
                .help("Target Project path [\".\" = current path]")
                .required(true)
                .takes_value(true) 
            )
            .arg(Arg::with_name("New Boilerplate name")
                .help("New Boilerplate name")
                .takes_value(true) 
            )
            .arg(Arg::with_name("force")
                .help("Overwrite if duplicates")
                .short("f")
                .long("force")
            )
        )
}

pub fn arg_init(matches: &ArgMatches) -> io::Result<()>{
    if let Some(ref matches) = matches.subcommand_matches("init") {
        if let Some(blog_name) = matches.value_of("Blog name") {
            fs::create_dir(blog_name)?;
            fs::create_dir(format!("{}/{}", blog_name, SOURCE_PATH));
            fs::create_dir(format!("{}/{}", blog_name, SOURCE_POSTS_PATH));
            fs::create_dir(format!("{}/{}", blog_name, SOURCE_LAYOUTS_PATH));
            fs::create_dir(format!("{}/{}", blog_name, DIST_POSTS_PATH));
            fs::File::create(format!("{}/{}/{}", blog_name, SOURCE_LAYOUTS_PATH, "header.html"));
            let mut layout_file   = fs::File::create(format!("{}/{}/{}", blog_name, SOURCE_LAYOUTS_PATH, "layout.html"))?;
            let mut articles_file = fs::File::create(format!("{}/{}/{}", blog_name, SOURCE_LAYOUTS_PATH, "articles.html"))?;

            let layout_content = "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
    <title>{{ page_title }}</title>
</head>
<body>
    {{ header }}
    {{ contents }}
</body>
</html>
            ";

            let articles_content = "
{% for article in articles %}
<div>
    <a href=\"{{ article.path }}\">{{ article.title }}</a>
</div>
{% endfor %}
            ";
            layout_file.write_all(layout_content.as_bytes())?;
            articles_file.write_all(articles_content.as_bytes())?;
        }
    }

    Ok(())
}

pub fn arg_new(matches: &ArgMatches) -> io::Result<()> {
    if let Some(ref matches) = matches.subcommand_matches("new") {
        if let Some(article_title) = matches.value_of("Article title") {
            let source_posts_path     = Path::new(SOURCE_POSTS_PATH);
            let date: DateTime<Local> = Local::now();

            let year_path  = source_posts_path.join(date.year().to_string());
            let month_path = year_path.join(date.month().to_string());
            let day_path   = month_path.join(date.day().to_string());

            if(!year_path.exists()) {
                fs::create_dir(year_path);
            }

            if(!month_path.exists()) {
                fs::create_dir(month_path);
            }

            if(!day_path.exists()) {
                fs::create_dir(&day_path);
            }

            let article_path = day_path.join(
                format!("{}_{}_{}_{}.md", date.hour(), date.minute(), date.second(), article_title)
            );

            fs::File::create(article_path)?;
        }
    }

    Ok(())
}

pub fn arg_generate(matches: &ArgMatches) -> io::Result<()> {
    if let Some(ref matches) = matches.subcommand_matches("generate") {
        utils::generate_html_files();
        utils::generate_index_html();
    }

    Ok(())
}