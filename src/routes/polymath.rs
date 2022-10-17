use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/polymath/how-to-present")]
async fn how_to_present() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/how_to_present", context.into_json())
}

#[get("/portals/polymath/how-to-become-a-polymath")]
async fn how_to_become_a_polymath() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/how_to_become_a_polymath",
        context.into_json(),
    )
}

#[get("/portals/polymath/how-to-read-a-book")]
async fn how_to_read_a_book() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/how_to_read_a_book", context.into_json())
}

#[get("/portals/polymath/lost-arts-of-polymath-education")]
async fn lost_arts() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/lost_arts_of_education",
        context.into_json(),
    )
}

#[get("/portals/polymath/towards-a-polymath-education")]
async fn towards_a_polymath_education() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/polymath/towards_a_polymath_education",
        context.into_json(),
    )
}

#[get("/portals/polymath/polymath-portal")]
async fn polymath_portal() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/polymath_portal", context.into_json())
}

#[get("/portals/polymath/slip-box")]
async fn slip_box() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/polymath/slip_box", context.into_json())
}

pub fn get_polymath_routes() -> Vec<Route> {
    routes![
        how_to_present,
        how_to_become_a_polymath,
        how_to_read_a_book,
        lost_arts,
        towards_a_polymath_education,
        polymath_portal,
        slip_box
    ]
}
