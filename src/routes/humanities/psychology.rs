use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/psychology/transactional-analysis")]
async fn ta() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/humanities/psychology/transactional_analysis",
        context.into_json(),
    )
}

pub fn get_psychology_routes() -> Vec<Route> {
    routes![ta]
}
