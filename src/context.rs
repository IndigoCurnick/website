use std::path::PathBuf;

use blog_tools::{get_blog, Blog};
use lazy_static::lazy_static;

pub static BLOG_ROOT: &str = "blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog(PathBuf::from(BLOG_ROOT), None, None);
}
