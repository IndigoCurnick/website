use rocket::Route;
use rocket_dyn_templates::Template;

use super::{
    basic_mathematics::get_basic_mathematics_routes, basic_physics::get_basic_physics_routes,
};

#[get("/portals/science/introduction-to-truth-tables")]
async fn truth_tables() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/science/introduction_to_truth_tables",
        context.into_json(),
    )
}

#[get("/portals/science/introduction-to-variables")]
async fn variables() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/science/introduction_to_variables",
        context.into_json(),
    )
}

#[get("/portals/science/science-portal")]
async fn science_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/science/science_portal", context.into_json())
}

pub fn get_science_routes() -> Vec<Route> {
    let basic_maths = get_basic_mathematics_routes();
    let basic_physics = get_basic_physics_routes();
    let science_routes = routes![truth_tables, variables, science_portal];

    return vec![basic_maths, basic_physics, science_routes].concat();
}
