
use std::sync::Arc;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use uuid::Uuid;

use ::repository::Repository;
use ::todo::Todo;

pub struct GETTodosHandler {
    repository: Arc<Repository<Todo>>
}

impl GETTodosHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> GETTodosHandler {
        GETTodosHandler {
            repository: repository,
        }
    }
}

impl Handler for GETTodosHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
         Ok(Response::with(status::Ok))
    }
}


pub struct POSTTodosHandler {
    repository: Arc<Repository<Todo>>
}

impl POSTTodosHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> POSTTodosHandler {
        POSTTodosHandler {
            repository: repository,
        }
    }
}

impl Handler for POSTTodosHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let id = Uuid::new_v4().to_hyphenated_string();
        let todo = Todo::new(id.clone(), String::from("The title"));
        self.repository.add(id, todo);
        Ok(Response::with(status::Created))
    }
}
