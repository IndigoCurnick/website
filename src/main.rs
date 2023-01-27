#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;
use std::path::PathBuf;

use blog::{get_blog_entries, Blog};
use context::{BLOG_ROOT, STATIC_BLOG_ENTRIES};
use database::{insert_to_database, pg_init};
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::routes;
use rocket::{catchers, Route};
use rocket_dyn_templates::Template;

use rocket::Request;
use routes::humanities::humanities::get_humanities_routes;
use routes::polymath::get_polymath_routes;
use routes::programming::get_programming_routes;
use routes::science::science::get_science_routes;

mod database;
mod routes;
mod utils;

mod blog;
mod context;

#[cfg(test)]
mod tests;

pub static DOMAIN: &str = "nathanielcurnick.xyz";

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

#[get("/index")]
async fn index() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/index".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("index", context.into_json())
}

#[get("/blog/hub")]
async fn blog_hub() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/blog/hub".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/blog_main", context.into_json())
}

#[get("/blogtemp")]
async fn blog_temp() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Template::render("blog", context.into_json())
}

fn get_blog_context() -> &'static Blog {
    return &STATIC_BLOG_ENTRIES;
}

#[get("/blog/<slug>")]
fn blog_article(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();
    let this_blog = match all_blogs.hash.get(&slug) {
        Some(x) => x,
        None => return None,
    };
    context.insert("blog", this_blog);
    Some(Template::render(
        format!("/blog/{}", slug),
        context.into_json(),
    ))
}

#[rocket::main]
async fn main() {
    // env::set_var("PG_URI", "0.0.0.0:5432");
    // env::set_var("PG_DB", "prod");
    // env::set_var("PG_PASSWORD", "KFUfDF2w7AkVEsg3");
    // env::set_var("PG_USERNAME", "postgress");

    println!("Booting up");
    if !cfg!(debug_assertions) {
        match pg_init().await {
            Ok(()) => {}
            Err(()) => panic!("Could not connect to the database"),
        };
    }

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
    let index_route = routes![index, blog_temp, blog_article];
    let science = get_science_routes();
    let polymath = get_polymath_routes();
    let humanities = get_humanities_routes();
    let programming = get_programming_routes();
    let blog_hub = routes![blog_hub];
    let all_routes = vec![
        index_route,
        blog_hub,
        science,
        polymath,
        humanities,
        programming,
    ];

    let flattened_routes = all_routes.concat();
    return flattened_routes;
}
