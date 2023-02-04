use rocket::Route;

use self::basic_mathematics::get_basic_mathematics_routes;

mod basic_mathematics;

pub fn get_mathematics_courses_routes() -> Vec<Route> {
    return get_basic_mathematics_routes();
}
