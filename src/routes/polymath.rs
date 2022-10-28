use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/portals/polymath/how-to-present")]
async fn how_to_present() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/how-to-present".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/how_to_present", context.into_json())
}

#[get("/portals/polymath/how-to-become-a-polymath")]
async fn how_to_become_a_polymath() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/how-to-become-a-polymath".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/how_to_become_a_polymath",
        context.into_json(),
    )
}

#[get("/portals/polymath/how-to-read-a-book")]
async fn how_to_read_a_book() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/how-to-read-a-book".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/how_to_read_a_book", context.into_json())
}

#[get("/portals/polymath/lost-arts-of-polymath-education")]
async fn lost_arts() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/lost-arts-of-polymath-education".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/lost_arts_of_education",
        context.into_json(),
    )
}

#[get("/portals/polymath/towards-a-polymath-education")]
async fn towards_a_polymath_education() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/towards-a-polymath-education".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/towards_a_polymath_education",
        context.into_json(),
    )
}

#[get("/portals/polymath/polymath-portal")]
async fn polymath_portal() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/polymath/polymath-portal".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/polymath_portal", context.into_json())
}

#[get("/portals/polymath/slip-box")]
async fn slip_box() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/portals/polymath/slip-box".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/slip_box", context.into_json())
}

pub fn get_polymath_routes() -> Vec<Route> {
    routes![
        how_to_present,
        how_to_become_a_polymath,
        how_to_read_a_book,
        lost_arts,
        towards_a_polymath_education,
        polymath_portal,
        slip_box
    ]
}
