use clap::{App, AppSettings, Arg, SubCommand, ArgMatches};
use chrono::prelude::*;

use std::fs;
use std::io;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use { SOURCE_PATH, SOURCE_POSTS_PATH, GENERATED_POSTS_PATH, LAYOUTS_POSTS_PATH };

pub fn init() {
    let matches = build().get_matches();
    arg_init(&matches);
    arg_new(&matches);
}

pub fn build() -> App<'static, 'static> {
    App::new("clapex")
        .version("0.1.0")                       
        .author("myname <myname@mail.com>")     
        .about("Clap Example CLI")              
        .help("sample positional argument")  
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
            fs::create_dir(format!("{}/{}", blog_name, GENERATED_POSTS_PATH));
            fs::create_dir(format!("{}/{}", blog_name, LAYOUTS_POSTS_PATH));
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

            println!("{:?}", year_path);
            println!("{:?}", month_path);
            println!("{:?}", day_path);
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