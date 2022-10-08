use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/history/commando")]
async fn commando() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/humanities/history/commando", context.into_json())
}

pub fn get_history_routes() -> Vec<Route> {
    routes![commando]
}
