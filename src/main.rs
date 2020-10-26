use warp::{http, Filter};

/*
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
*/

// fn post(request: &mut Request) -> IronResult<Response> {
async fn post(x: u64, y: u64, z: u64) -> Result<impl warp::Reply, warp::Rejection> {
    //let ref x = request.extensions.get::<Router>().unwrap().find("x").unwrap();
    //let ref y = request.extensions.get::<Router>().unwrap().find("y").unwrap();
    //let ref z = request.extensions.get::<Router>().unwrap().find("z").unwrap();

    // Ok(Response::with((status::Ok, format!("post position | x = {} | y = {} | z = {}", x, y, z))))
    Ok(warp::reply::with_status(
        format!("post position | x = {} | y = {} | z = {}", x, y, z),
        http::StatusCode::CREATED
    ))
}

/*
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
*/

#[tokio::main]
async fn main() {
    // https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

    // let mut router = Router::new();

    // router.get("/", health, "health");
    let health = warp::path::end().map(|| "health");

    // router.post("/", consume, "consume");

    // router.get("/:x/:y/:z", get, "get");
/*    let getPoint = warp::path!(u64 / u64 / u64)
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get);
*/
    // router.post("/:x/:y/:z", post, "post");
    let post_point = warp::path!(u64 / u64 / u64)
        .and(warp::path::end())
        .and(warp::post())
//        .and(json_body())
        .and_then(post);
/*
    // router.put("/:x/:y/:z", put, "put");
    let postPoint = warp::path!(u64 / u64 / u64)
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body())
        .and_then(posto);

    // router.delete("/:x/:y/:z", delete, "delete");
    let postPoint = warp::path!(u64 / u64 / u64)
        .and(warp::path::end())
        .and(warp::post())
        .and_then(posto);

    // router.options("/:x/:y/:z", options, "options");
    let optionsPoint = warp::path!(u64 / u64 / u64)
        .and(warp::path::end())
        .and(warp::options())
        .and_then(posto);
*/
    //Iron::new(router).http("0.0.0.0:8080").unwrap();
    let routes = health.or(post_point);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
