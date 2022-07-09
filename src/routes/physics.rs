use rocket_dyn_templates::Template;

#[get("/physics/kinematics")]
pub async fn kinematics() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("physics/kinematics", context.into_json())
}
