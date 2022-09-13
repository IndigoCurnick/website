use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals")]
async fn portals() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/portal_main", context.into_json())
}

#[get("/portals/philosophy/philosophy_portal")]
async fn philosophy_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/philosophy/philosophy_portal", context.into_json())
}

#[get("/portals/economics/economic_portal")]
async fn economic_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/economics/economic_portal", context.into_json())
}

#[get("/portals/history/history_portal")]
async fn history_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/history/history_portal", context.into_json())
}

#[get("/portals/politics/politics_portal")]
pub async fn politics_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/politics/politics_portal", context.into_json())
}

#[get("/portals/science/science_portal")]
async fn science_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/science/science_portal", context.into_json())
}

#[get("/portals/programming/programming_portal")]
async fn programming_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/programming/programming_portal",
        context.into_json(),
    )
}

#[get("/portals/polymath/polymath_portal")]
async fn polymath_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/polymath_portal", context.into_json())
}

pub fn get_portal_routes() -> Vec<Route> {
    return routes![
        portals,
        philosophy_portal,
        economic_portal,
        history_portal,
        politics_portal,
        science_portal,
        programming_portal,
        polymath_portal
    ];
}
