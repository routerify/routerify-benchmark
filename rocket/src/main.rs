#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, World!")
}

#[tokio::main]
async fn main() {
    rocket::custom(rocket::Config::figment().merge(("port", 8081)))
        .mount("/", routes![hello])
        .launch()
        .await
        .expect("Run Rocket");
}
