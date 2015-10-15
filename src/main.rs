extern crate spell_core;
extern crate iron;
extern crate router;

use iron::status;
use iron::{Iron, Request, Response, IronResult};
use router::Router;
use spell_core::SpellCorrector;
use std::io::Read;
use std::sync::{Arc, Mutex};

fn say_pong(req: &mut Request) -> IronResult<Response> {
    println!("Running ping handler, URL path: {}", req.url.path.join("/"));
    Ok(Response::with((status::Ok, "Pong!")))
}

fn spell_check(request: &mut Request, corrector: &SpellCorrector) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    println!("{}", payload);
    Ok(Response::with((status::Ok, corrector.correct(payload))))
}

fn main() {
    let corrector = Arc::new(Mutex::new(SpellCorrector::new()));

    let mut router = Router::new();
    router.get("/ping", say_pong);
    router.post("/correct", move |r : &mut Request| spell_check(r, &corrector.lock().unwrap()));
    println!("Listening on port 3000");
    Iron::new(router).http("127.0.0.1:3000").unwrap();
}
