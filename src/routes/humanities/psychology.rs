use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/portals/psychology/transactional-analysis")]
async fn ta() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/psychology/transactional-analysis".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/humanities/psychology/transactional_analysis",
        context.into_json(),
    )
}

pub fn get_psychology_routes() -> Vec<Route> {
    routes![ta]
}
