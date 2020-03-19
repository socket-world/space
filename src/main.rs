extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "health")))
}

fn consume(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "consume")))
}

fn get(request: &mut Request) -> IronResult<Response> {
    let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    Ok(Response::with((status::Ok, format!("get position | x = {} | y = {} | z = {}", x, y, z))))
}

fn post(request: &mut Request) -> IronResult<Response> {
    let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    Ok(Response::with((status::Ok, format!("post position | x = {} | y = {} | z = {}", x, y, z))))
}

fn put(request: &mut Request) -> IronResult<Response> {
    let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    Ok(Response::with((status::Ok, format!("put position | x = {} | y = {} | z = {}", x, y, z))))
}

fn delete(request: &mut Request) -> IronResult<Response> {
    let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    Ok(Response::with((status::Ok, format!("delete position | x = {} | y = {} | z = {}", x, y, z))))
}

fn options(request: &mut Request) -> IronResult<Response> {
    let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    Ok(Response::with((status::Ok, format!("options position | x = {} | y = {} | z = {}", x, y, z))))
}

fn main() {
    let mut router = Router::new();

    router.get("/", health, "health");
    router.post("/", consume, "consume");

    router.get("/:x/:y/:z", get, "get");
    router.post("/:x/:y/:z", post, "post");
    router.put("/:x/:y/:z", put, "put");
    router.delete("/:x/:y/:z", delete, "delete");
    router.options("/:x/:y/:z", options, "options");

    Iron::new(router).http("0.0.0.0:8080").unwrap();
}
