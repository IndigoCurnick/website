use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use glob::glob;
use serde::{Deserialize, Serialize};

type Slug = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub hash: HashMap<Slug, BlogEntry>,
    pub entries: Vec<BlogEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogEntry {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub html: String,
    pub slug: Slug,
    pub tags: Vec<String>,
}

impl BlogEntry {
    pub fn new(json: BlogJson, html: String) -> Self {
        return BlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            html: html,
            slug: json.slug,
            tags: json.tags,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogJson {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub slug: String,
    pub tags: Vec<String>,
}

fn get_blog_paths(base: PathBuf) -> Result<Vec<PathBuf>, String> {
    if !base.is_dir() {
        panic!("Blog base is not dir");
    }

    let mut html_files: Vec<PathBuf> = vec![];

    let base_str = base.to_str().unwrap();
    let glob_str = format!("{}/**/*.html.tera", base_str);
    for entry in glob(&glob_str).expect("Failed to read glob string") {
        match entry {
            Ok(path) => {
                let pathbuf = PathBuf::from(path);
                html_files.push(pathbuf);
            }
            Err(y) => panic!("{}", y),
        }
    }
    return Ok(html_files);
}

pub fn get_blog_entries(base: PathBuf) -> Blog {
    // TODO: Error
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<Slug, BlogEntry> = HashMap::new();
    let mut entires: Vec<BlogEntry> = vec![];

    for blog in blog_paths {
        let mut json_path = blog.parent().unwrap().to_path_buf();
        let name_split: Vec<&str> = blog
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect();
        let name = format!("{}.json", name_split[0]);

        json_path.push(name);
        println!("json path is {}", json_path.display());
        let json_text = fs::read_to_string(json_path).unwrap();

        let json_data: BlogJson = serde_json::from_str(&json_text).unwrap();

        let html = fs::read_to_string(blog).unwrap();

        let blog_entry = BlogEntry::new(json_data, html);

        hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
        entires.push(blog_entry);
    }

    return Blog {
        hash: hashes,
        entries: entires,
    };
}

#[test]
fn test_get_blog_entries() {
    let blog_entries = get_blog_entries(PathBuf::from("blog"));

    println!("{:?}", blog_entries);
}

#[test]
fn test_get_blog_files() {
    let html_files = get_blog_paths(PathBuf::from("blog")).unwrap();

    println!("{:?}", html_files);
}
