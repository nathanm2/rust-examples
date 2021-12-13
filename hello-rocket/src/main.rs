#[macro_use]
extern crate rocket;

#[get("/world")]
fn world2() -> &'static str {
    "Hello, world 2"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn blah() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hi", routes![world2])
}
