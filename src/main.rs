#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::BadRequest,
    Request,
};
use rocket_contrib::json::Json;

#[derive(FromForm, Serialize)]
struct Numbers {
    n1: i32,
    n2: i32,
}

#[derive(Serialize)]
struct ErrorMessage<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct NumbersResult {
    n1: i32,
    n2: i32,
    add: i64,
    sub: i64,
}

#[derive(Debug, Serialize)]
struct InvalidNumber<'a> {
    message: &'a str,
}

impl<'a, 'r> FromRequest<'a, 'r> for Numbers {
    type Error = InvalidNumber<'a>;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let n1 = request.get_query_value("n1").and_then(|r| r.ok());
        let n2 = request.get_query_value("n2").and_then(|r| r.ok());
        match (n1, n2) {
            (Some(x), Some(y)) => Outcome::Success(Numbers { n1: x, n2: y }),
            (None, _) => Outcome::Failure((
                Status::BadRequest,
                InvalidNumber {
                    message: "First number was invalid",
                },
            )),
            (_, None) => Outcome::Failure((
                Status::BadRequest,
                InvalidNumber {
                    message: "Second number was invalid",
                },
            )),
        }
    }
}

#[get("/math")]
fn math<'a>(
    numbers: Result<Numbers, InvalidNumber>,
) -> Result<Json<NumbersResult>, BadRequest<Json<InvalidNumber>>> {
    match numbers {
        Ok(Numbers { n1, n2 }) => Ok(Json(NumbersResult {
            n1,
            n2,
            add: n1 as i64 + n2 as i64,
            sub: n1 as i64 - n2 as i64,
        })),
        Err(e) => Err(BadRequest(Some(Json(e)))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("{} is not a valid path", req.uri())
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![math])
        .register(catchers![not_found])
        .launch();
}
