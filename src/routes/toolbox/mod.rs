use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/toolbox/geodesics")]
pub async fn geodesics() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("toolbox/geodesics", context.into_json())
}

#[get("/toolbox")]
pub async fn index() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("toolbox/index", context.into_json())
}

pub fn get_toolbox_routes() -> Vec<Route> {
    return routes![geodesics, index];
}
