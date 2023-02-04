use rocket::Route;

use self::basic_physics::get_basic_physics_routes;

mod basic_physics;

pub fn get_science_courses() -> Vec<Route> {
    return get_basic_physics_routes();
}
