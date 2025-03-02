use std::{fs, path::PathBuf};

use blog_tools::{
    high::{get_high_blog, HighBlog},
    sitemap::SitemapOptions,
};
use lazy_static::lazy_static;

pub static BLOG_ROOT: &str = "blog";
pub static BLOG_URL: &str = "https://indigocurnick.xyz";

pub fn generate_blog() -> HighBlog {
    get_high_blog(
        PathBuf::from(BLOG_ROOT),
        None,
        None,
        &BLOG_URL.to_string(),
        &SitemapOptions {
            include_tags: true,
            default_priority: 0.7,
            sitemap_base: Some(fs::read_to_string("xml/sitemap-base.xml").unwrap()),
            ..Default::default()
        }
    )
    .unwrap()
}

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: HighBlog = generate_blog();
}
