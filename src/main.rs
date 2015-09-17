extern crate uuid;
extern crate iron;
extern crate router;

mod repository;
mod todo;
mod handlers;

use std::sync::Arc;

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

    Iron::new(router).http("localhost:3000").unwrap();
}
