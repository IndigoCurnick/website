use std::{collections::HashMap, ffi::OsStr, fs, path::PathBuf};

use chrono::NaiveDate;
use markdown::to_html;
use serde::{Deserialize, Serialize};
use std::io;

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

fn get_blog_paths(base: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut markdown_files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for file in fs::read_dir(path)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                continue;
            }
            if path.extension().and_then(OsStr::to_str).unwrap_or("") == "md" {
                markdown_files.push(path);
            }
        }
    }
    Ok(markdown_files)
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

        let markdown = fs::read_to_string(blog).unwrap();

        let html = to_html(&markdown);

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
