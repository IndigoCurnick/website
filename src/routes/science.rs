use rocket::Route;
use rocket_dyn_templates::Template;

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

pub fn get_science_routes() -> Vec<Route> {
    routes![truth_tables, variables]
}
