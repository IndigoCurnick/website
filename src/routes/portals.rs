use rocket_dyn_templates::Template;

#[get("/portals")]
pub async fn portals() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/portal_main", context.into_json())
}

#[get("/portals/philosophy_portal")]
pub async fn philosophy_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/philosophy_portal", context.into_json())
}

#[get("/portals/economic_portal")]
pub async fn economic_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/economic_portal", context.into_json())
}

#[get("/portals/history_portal")]
pub async fn history_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/history_portal", context.into_json())
}

#[get("/portals/politics_portal")]
pub async fn politics_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/politics_portal", context.into_json())
}

#[get("/portals/science_portal")]
pub async fn science_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/science_portal", context.into_json())
}

#[get("/portals/urban_portal")]
pub async fn urban_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/urban_portal", context.into_json())
}
