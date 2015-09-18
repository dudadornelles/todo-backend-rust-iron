#[macro_use] extern crate rustc_serialize;
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

use iron::prelude::*;
use router::Router;

use ::handlers::*;
use ::repository::Repository;
use ::todo::Todo;

fn main() {
    let mut router = Router::new();
    let repository: Arc<Repository<Todo>> = Arc::new(Repository::new());

    router.get("/todos", GETTodosHandler::new(repository.clone()));
    router.post("/todos", POSTTodosHandler::new(repository.clone()));
    router.delete("/todos", DELETETodosHandler::new(repository.clone()));

    router.get("/todos/:id", GETTodoHandler::new(repository.clone()));
    router.patch("/todos/:id", PATCHTodoHandler::new(repository.clone()));
    router.delete("/todos/:id", DELETETodoHandler::new(repository.clone()));

    fn get_server_port() -> u16 {
        let port_str = env::var("PORT").unwrap_or(String::new());
        FromStr::from_str(&port_str).unwrap_or(8080)
    }

    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}
