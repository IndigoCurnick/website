use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/politics/a-defence-of-machiavelli")]
async fn a_defence_of_mach() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/politics/a_defence_of_mach", context.into_json())
}

#[get("/portals/politics/war-that-will-end-war")]
async fn the_war_that_will_end_war() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/politics/the_war_that_will_end_war",
        context.into_json(),
    )
}

#[get("/portals/politics/beyond-left-right")]
async fn beyond_left_right() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/politics/beyond_left_right", context.into_json())
}

pub fn get_politics_routes() -> Vec<Route> {
    routes![
        a_defence_of_mach,
        the_war_that_will_end_war,
        beyond_left_right
    ]
}
