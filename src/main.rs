#[macro_use]
extern crate rocket;

use std::{env, fs};

use blog_tools::high::{HighBlog, HighBlogEntry};
use blog_tools::Blog;
use context::STATIC_BLOG_ENTRIES;

use rocket::fs::{relative, FileServer};
use rocket::response::content::RawXml;
use rocket::response::Redirect;
use rocket::routes;
use rocket::{catchers, Route};
use rocket_dyn_templates::Template;

use rocket::Request;

use routes::courses::kalman::kalman::get_kalman_courses;
use routes::courses::mathematics::get_mathematics_courses_routes;
use routes::courses::science::get_science_courses;
use routes::toolbox::get_toolbox_routes;

mod context;
mod routes;
mod utils;

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(index))
}

#[catch(500)]
async fn error(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(index))
}

#[get("/sitemap.xml")]
async fn sitemap() -> RawXml<String> {
    let blog = get_blog_context();

    return RawXml(blog.sitemap.clone());
}

#[get("/index")]
async fn index() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();

    let blog_context = get_blog_context();
    context.insert("tags", &blog_context.tags);
    context.insert("blog", get_blog_context());
    Template::render("index", context.into_json())
}

#[get("/blog")]
async fn blog_index() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Template::render("blog_index", context.into_json())
}

#[get("/courses")]
async fn courses_hub() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/courses", context.into_json())
}

fn get_blog_context() -> &'static HighBlog {
    return &STATIC_BLOG_ENTRIES;
}

#[get("/blog/<date>/<slug>", rank = 2)]
fn blog_article(date: String, slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();
    let this_slug = format!("{}/{}", date, slug);
    let this_blog = match all_blogs.hash.get(&this_slug) {
        Some(x) => x,
        None => return None,
    };
    context.insert("blog", this_blog);
    Some(Template::render("blog", context.into_json()))
}

#[get("/blog/tag/<slug>")]
fn tag_page(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();

    let mut these_blogs: Vec<&HighBlogEntry> = vec![];

    for blog in &all_blogs.entries {
        if blog.get_tags().contains(&slug) {
            these_blogs.push(&blog);
        }
    }

    context.insert("blogs", &these_blogs);
    context.insert("tag", &slug);
    Some(Template::render("tags", context.into_json()))
}

#[derive(Responder)]
#[response(status = 200, content_type = "text")]
struct RawOkText(&'static str);

#[get("/ping")]
fn ping() -> RawOkText {
    return RawOkText("pong");
}

#[rocket::main]
async fn main() {
    println!("Booting up");

    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .unwrap();

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));
    // .merge(("secret_key", secret_key));

    if let Err(e) = rocket::custom(figment)
        .mount("/", FileServer::from(relative!("assets/")))
        .register("/", catchers![not_found, error])
        .attach(Template::fairing())
        // .attach(config)
        .mount("/", get_all_routes())
        // .manage(bucket_info)
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}

fn get_all_routes() -> Vec<Route> {
    let index_route = routes![
        index,
        blog_index,
        blog_article,
        tag_page,
        courses_hub,
        sitemap
    ];
    let maths_courses = get_mathematics_courses_routes();
    let science_courses = get_science_courses();
    let kalman_courses = get_kalman_courses();
    let util_routes = routes![ping];

    let toolbox_routes = get_toolbox_routes();

    let all_routes = vec![
        index_route,
        maths_courses,
        science_courses,
        kalman_courses,
        util_routes,
        toolbox_routes,
    ];

    let flattened_routes = all_routes.concat();
    return flattened_routes;
}
