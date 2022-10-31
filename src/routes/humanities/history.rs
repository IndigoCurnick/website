use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/blog/history/commando")]
async fn commando() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/blog/history/commando".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/humanities/history/commando", context.into_json())
}

pub fn get_history_routes() -> Vec<Route> {
    routes![commando]
}
