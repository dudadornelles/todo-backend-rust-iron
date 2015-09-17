extern crate bodyparser;

use std::sync::Arc;

use rustc_serialize::json;
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
        let all = self.repository.all();
        let all_json = json::encode(&all).unwrap();
        Ok(Response::with((status::Ok, all_json)))
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
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        return match json_body {
            Ok(Some(json_body)) => {
                let id = Uuid::new_v4().to_hyphenated_string();
                let obj = json_body.as_object().unwrap();
                let todo = Todo::new(id.clone(), String::from(obj.get("title").unwrap().as_string().unwrap()));
                let todo = self.repository.add(id, todo);
                Ok(Response::with((status::Created, json::encode(&todo).unwrap())))
            }
            Ok(None) => panic!("No body"),
            Err(err) => panic!("Error: {:?}", err)
        }
    }
}
