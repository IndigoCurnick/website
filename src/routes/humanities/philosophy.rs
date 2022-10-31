use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

// =============================================================================
// Presocratics
// =============================================================================
#[get("/blog/philosophy/pre-socratics/preamble")]
async fn preamble() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/preamble".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/preamble",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part1")]
async fn part1() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part1".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part1",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part2")]
async fn part2() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part2".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part2",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part3")]
async fn part3() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part3".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part3",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part4")]
async fn part4() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part4".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part4",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part5")]
async fn part5() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part5".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part5",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part6")]
async fn part6() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part6".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part6",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/part7")]
async fn part7() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/part7".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/part7",
        context.into_json(),
    )
}

#[get("/blog/philosophy/pre-socratics/socrates")]
async fn socrates() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/pre-socratics/socrates".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/pre_socratics/socrates",
        context.into_json(),
    )
}

fn get_presocratic_routes() -> Vec<Route> {
    return routes![preamble, part1, part2, part3, part4, part5, part6, part7, socrates];
}

// =============================================================================
// Other Philo
// =============================================================================

#[get("/blog/philosophy/very-short-intro-logic")]
async fn very_short_intro_logic() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/very-short-intro-logic".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/very_short_intro_logic",
        context.into_json(),
    )
}

#[get("/blog/philosophy/definition-of-art")]
async fn definition_of_art() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/definition-of-art".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/definition_of_art",
        context.into_json(),
    )
}

#[get("/blog/philosophy/social-contract")]
async fn social_contract() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/social-contract".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/social_contract",
        context.into_json(),
    )
}

#[get("/blog/philosophy/what-can-we-know")]
async fn what_can_we_know() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/philosophy/what-can-we-know".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/philosophy/what_can_we_know",
        context.into_json(),
    )
}

pub fn get_philo_routes() -> Vec<Route> {
    let presocratic = get_presocratic_routes();
    let philo_routes = routes![
        very_short_intro_logic,
        definition_of_art,
        social_contract,
        what_can_we_know
    ];

    return vec![presocratic, philo_routes].concat();
}
