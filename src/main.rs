#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use database::{insert_to_database, pg_init};
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::routes;
use rocket::{catchers, Route};
use rocket_dyn_templates::Template;

use rocket::Request;
use routes::basic_mathematics::get_basic_mathematics_routes;
use routes::basic_physics::get_physics_routes;
use routes::economics::get_econ_routes;
use routes::history::get_history_routes;
use routes::philosophy::{get_philo_routes, get_presocratic_routes};
use routes::politics::get_politics_routes;
use routes::polymath::get_polymath_routes;
use routes::portals::get_portal_routes;
use routes::programming::get_programming_routes;
use routes::science::get_science_routes;
mod database;
mod routes;
mod utils;

pub static DOMAIN: &str = "nathanielcurnick.xyz";

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Redirect {
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

#[rocket::main]
async fn main() {
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
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        // .attach(config)
        // TODO! Find a better way to expose this many endpoints.
        .mount("/", get_all_routes())
        // .manage(bucket_info)
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}

fn get_all_routes() -> Vec<Route> {
    let index_route = routes![index];
    let portals_routes = get_portal_routes();
    let physics = get_physics_routes();
    let presocratics = get_presocratic_routes();
    let philo = get_philo_routes();
    let econ = get_econ_routes();
    let politics = get_politics_routes();
    let polymath = get_polymath_routes();
    let history = get_history_routes();
    let science = get_science_routes();
    let programming = get_programming_routes();
    let basic_maths = get_basic_mathematics_routes();

    let all_routes = vec![
        index_route,
        portals_routes,
        physics,
        presocratics,
        philo,
        econ,
        politics,
        polymath,
        history,
        science,
        programming,
        basic_maths,
    ];

    let flattened_routes = all_routes.concat();
    return flattened_routes;
}
