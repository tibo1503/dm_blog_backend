#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Articles
#[get("/")]
fn article() -> String {
    format!("List of articles")
}

#[get("/<id>")]
fn article_id(id: u64) -> String {
    format!("Article id: {}", id)
}

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        .mount(format!("{}{}", api_url, "/"), routes![index])
        .mount(format!("{}{}", api_url, "/auth"), routes![index])
        .mount(format!("{}{}", api_url, "/article"), routes![article, article_id])
}
