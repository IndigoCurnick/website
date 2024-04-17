use std::path::PathBuf;

use blog_tools::high::{get_high_blog, HighBlog};
use lazy_static::lazy_static;

pub static BLOG_ROOT: &str = "blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: HighBlog =
        get_high_blog(PathBuf::from(BLOG_ROOT), None, None).unwrap();
}
