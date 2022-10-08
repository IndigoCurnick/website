use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/programming/handling-errors-in-rust")]
async fn handling_errors() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/programming/handling_errors", context.into_json())
}

#[get("/portals/programming/how-to-refactor")]
async fn refactor() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/programming/how_to_refactor", context.into_json())
}

#[get("/portals/programming/joy-of-programming")]
async fn joy() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/programming/joy_of_programming",
        context.into_json(),
    )
}

#[get("/portals/programming/lifetime-ownership-rust")]
async fn lifetimes() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/programming/lifetime_ownership",
        context.into_json(),
    )
}

#[get("/portals/programming/i-hate-oo")]
async fn i_hate_oo() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/programming/i_hate_oo", context.into_json())
}

#[get("/portals/programming/programming-portal")]
async fn programming_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/programming/programming_portal",
        context.into_json(),
    )
}

pub fn get_programming_routes() -> Vec<Route> {
    routes![
        handling_errors,
        refactor,
        joy,
        lifetimes,
        i_hate_oo,
        programming_portal
    ]
}
