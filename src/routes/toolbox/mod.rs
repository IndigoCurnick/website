use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/toolbox/haversine")]
pub async fn haversine() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/sin-wave".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("toolbox/haversine", context.into_json())
}

pub fn get_toolbox_routes() -> Vec<Route> {
    return routes![haversine];
}
