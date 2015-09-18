#[macro_use] extern crate rustc_serialize;
extern crate unicase;
extern crate mount;
extern crate bodyparser;
extern crate uuid;
extern crate iron;
extern crate router;

mod repository;
mod todo;
mod handlers;

use std::sync::Arc;
use std::env;
use std::str::FromStr;

use iron::headers;
use iron::method::Method::*;
use iron::prelude::*;
use iron::AfterMiddleware;
use mount::Mount;
use router::Router;
use unicase::UniCase;

use ::handlers::*;
use ::repository::Repository;
use ::todo::Todo;

struct CORS;

impl AfterMiddleware for CORS {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(
                vec![UniCase("accept".to_string()),
                UniCase("content-type".to_string())]));
        res.headers.set(headers::AccessControlAllowMethods(
                vec![Get,Head,Post,Delete,Options,Put,Patch]));
        Ok(res)
    }
}

fn main() {
    let mut router = Router::new();
    let repository: Arc<Repository<Todo>> = Arc::new(Repository::new());

    router.get("/todos", GETTodosHandler::new(repository.clone()));
    router.post("/todos", POSTTodosHandler::new(repository.clone()));
    router.delete("/todos", DELETETodosHandler::new(repository.clone()));

    router.get("/todos/:id", GETTodoHandler::new(repository.clone()));
    router.patch("/todos/:id", PATCHTodoHandler::new(repository.clone()));
    router.delete("/todos/:id", DELETETodoHandler::new(repository.clone()));

    let mut mount = Mount::new();
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    chain.link_after(CORS);

    fn get_server_port() -> u16 {
        let port_str = env::var("PORT").unwrap_or(String::new());
        FromStr::from_str(&port_str).unwrap_or(8080)
    }

    Iron::new(chain).http(("0.0.0.0", get_server_port())).unwrap();
}
