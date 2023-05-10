use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/courses/kalman-filters/notation")]
pub async fn notation() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/notation".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/kalman-filter-notation",
        context.into_json(),
    )
}

pub fn get_kalman_courses() -> Vec<Route> {
    return routes![notation];
}
