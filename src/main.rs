#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::catchers;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::routes;
use rocket_dyn_templates::Template;

use rocket::Request;

mod routes;

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(index))
}

#[get("/index")]
async fn index() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("index", context.into_json())
}

#[rocket::main]
async fn main() {
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
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        // .attach(config)
        // TODO! Find a better way to expose this many endpoints.
        .mount(
            "/",
            routes![
                index,
                routes::portals::portals,
                routes::portals::philosophy_portal,
                routes::portals::economic_portal,
                routes::portals::history_portal,
                routes::portals::politics_portal,
                routes::portals::science_portal,
                routes::portals::urban_portal,
                routes::physics::kinematics
            ],
        )
        // .manage(bucket_info)
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}
