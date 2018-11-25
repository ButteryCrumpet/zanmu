extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};

#[derive(Serialize)]
struct Thing {
    name: &'static str,
}

impl Responder for Thing {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(req: &HttpRequest) -> impl Responder {
    Thing { name: "?" }
}

fn server() {
    server::new(
        || App::new()
            .resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}