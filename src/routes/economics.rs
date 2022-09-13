use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/economics/after-the-welfare-state")]
async fn after_the_welfare_state() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/economics/after_the_welfare_state",
        context.into_json(),
    )
}

#[get("/portals/economics/british-welfare-state")]
async fn british_welfare_state() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/economics/british_welfare_state",
        context.into_json(),
    )
}

#[get("/portals/economics/bourgeois-virtues")]
async fn bourgeois_virtues() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/economics/bourgeois_virtues", context.into_json())
}

pub fn get_econ_routes() -> Vec<Route> {
    routes![
        after_the_welfare_state,
        british_welfare_state,
        bourgeois_virtues
    ]
}
