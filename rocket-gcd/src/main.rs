#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::http::ContentType;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    debug_assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let tmp = b;
            b = a;
            a = tmp;
        }
        b = b % a;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(3 * 5, 5 * 5), 5);
    assert_eq!(gcd(14, 15), 1);
}

#[derive(FromForm)]
struct Inputs {
    #[field(validate = range(1..))]
    n: u64,
    #[field(validate = range(1..))]
    m: u64,
}

#[get("/")]
fn get_gcd() -> (ContentType, &'static str) {
    (
        ContentType::HTML,
        r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
    <input type="text" name="n"/>
    <input type="text" name="m"/>
    <button type="submit">Compute GCD</button>
</form>"#,
    )
}

#[post("/gcd", data = "<inputs>")]
fn post_gcd(inputs: Form<Inputs>) -> (ContentType, String) {
    (
        ContentType::HTML,
        format!(
            "gcd of {} and {} is {}",
            inputs.n,
            inputs.m,
            gcd(inputs.n, inputs.m)
        ),
    )
}

#[catch(422)]
fn bad_parms() -> String {
    format!("Bad parameters!")
}

#[launch]
fn entry() -> _ {
    rocket::build()
        .mount("/", routes![get_gcd])
        .mount("/", routes![post_gcd])
        .register("/", catchers![bad_parms])
}
