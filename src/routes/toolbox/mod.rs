use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/toolbox/geodesics")]
pub async fn geodesics() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/toolbox/geodesics".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("toolbox/geodesics", context.into_json())
}

#[get("/toolbox")]
pub async fn index() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/toolbox".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("toolbox/index", context.into_json())
}

pub fn get_toolbox_routes() -> Vec<Route> {
    return routes![geodesics, index];
}
