use rocket::Route;
use rocket_dyn_templates::Template;

use super::{
    economics::get_econ_routes, history::get_history_routes, philosophy::get_philo_routes,
    politics::get_politics_routes, psychology::get_psychology_routes,
};

#[get("/portals/humanities/humanities-portal")]
async fn humanities_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/humanities/humanities_portal", context.into_json())
}

pub fn get_humanities_routes() -> Vec<Route> {
    let econ_routes = get_econ_routes();
    let history_routes = get_history_routes();
    let philosophy_routes = get_philo_routes();
    let politics_routes = get_politics_routes();
    let psychology_routes = get_psychology_routes();
    let humanities_routes = routes![humanities_portal];

    return vec![
        econ_routes,
        history_routes,
        philosophy_routes,
        politics_routes,
        psychology_routes,
        humanities_routes,
    ]
    .concat();
}
