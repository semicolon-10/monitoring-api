use rand::{Rng, thread_rng};
use rocket::{Build, Config, get, launch, Rocket, routes};
use rocket::http::Status;
use rocket_prometheus::PrometheusMetrics;

// Monitoring...!

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/health")]
fn health() -> Status {
    let number = thread_rng().gen_range(1..10);
    match number {
        1 => Status::Ok,
        2 => Status::BadRequest,
        3 => Status::TooManyRequests,
        _ => Status::NotFound
    }
}

#[launch]
fn rocket() -> Rocket<Build> {

    let prometheus = PrometheusMetrics::new();

    let config = Config {
        port: 4000,
        ..Config::default()
    };

    rocket::custom(config)
        .attach(prometheus.clone())
        .mount("/", routes![index, health])
        .mount("/metrics", prometheus)
}
