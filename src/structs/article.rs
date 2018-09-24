use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::{ Write, BufReader };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use std::ffi::OsString;

#[derive(Serialize, Debug, Clone)]
pub struct Article {
    path: Box<Path>,
    title: String,
    title_raw: String,
    raw_content: String
}

impl Article {
    pub fn new_from_DirEntry(entry: DirEntry) -> Article {
        let entry_path  = entry.path();
        let title       = Article::get_title_from_file_name(entry.file_name());
        let title_split_by_under_bar: Vec<&str>   = title.split('_').collect();
        let title_raw: Vec<&str>   = title_split_by_under_bar[3].split('.').collect();;
        let raw_content = Article::read_content(&entry_path).unwrap();

        Article {
            path: entry_path.into_boxed_path(),
            title: title.clone(),
            title_raw: title_raw[0].to_string(),
            raw_content
        }
    }

    fn get_title_from_file_name(full_file_name: OsString) -> String {
        let str_file_name        = full_file_name.into_string().unwrap();
        let file_name: Vec<&str> = str_file_name.split(".md").collect();

        String::from(file_name[0])
    }

    fn read_content(path: &PathBuf) -> io::Result<String> {
        let mut buf_reader  = BufReader::new(fs::File::open(path)?);
        let mut md_contents = String::new();
        buf_reader.read_to_string(&mut md_contents)?;

        Ok(md_contents)
    }
}