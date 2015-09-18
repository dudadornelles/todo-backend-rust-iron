extern crate bodyparser;

use std::sync::Arc;

use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use router::Router;
use uuid::Uuid;

use ::repository::Repository;
use ::todo::Todo;


// == GET /todos
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

// == POST /todos
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
                let json_object = json_body.as_object().unwrap();
                
                let new_title: String = { 
                    if json_object.get("title").is_some() {
                        String::from(json_object.get("title").unwrap().as_string().unwrap())
                    } else { 
                        String::from("") 
                    }
                };

                let new_order: u64 = {
                    if json_object.get("order").is_some() {
                        json_object.get("order").unwrap().as_u64().unwrap()
                    } else { 
                        0
                    }
                };

                let todo = Todo::new(id.clone(), new_title, false, new_order);
                let todo = self.repository.add(id, todo);
                Ok(Response::with((status::Created, json::encode(&todo).unwrap())))
            }
            Ok(None) => panic!("No body"),
            Err(err) => panic!("Error: {:?}", err)
        }
    }
}

// == DELETE /todos
pub struct DELETETodosHandler {
    repository: Arc<Repository<Todo>>
}

impl DELETETodosHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> DELETETodosHandler {
        DELETETodosHandler {
            repository: repository,
        }
    }
}

impl Handler for DELETETodosHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.repository.delete_all();
        Ok(Response::with(status::Ok))
    }
}

// == GET /todos/:id
pub struct GETTodoHandler {
    repository: Arc<Repository<Todo>>
}

impl GETTodoHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> GETTodoHandler {
        GETTodoHandler {
            repository: repository,
        }
    }
}

impl Handler for GETTodoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let todo = self.repository.get(String::from(id));
        Ok(Response::with((status::Ok, json::encode(&todo).unwrap())))
    }
}

// == PATCH /todos/:id
pub struct PATCHTodoHandler {
    repository: Arc<Repository<Todo>>
}

impl PATCHTodoHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> PATCHTodoHandler {
        PATCHTodoHandler {
            repository: repository,
        }
    }
}

impl Handler for PATCHTodoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        return match json_body {
            Ok(Some(json_body)) => {
                let id = String::from(req.extensions.get::<Router>().unwrap().find("id").unwrap());
                let json_object = json_body.as_object().unwrap();
                let old_todo = self.repository.get(id.clone());

                let new_title: String = { 
                    if json_object.get("title").is_some() {
                        String::from(json_object.get("title").unwrap().as_string().unwrap())
                    } else { 
                        old_todo.title.clone() 
                    }
                };

                let new_completed: bool = {
                    if json_object.get("completed").is_some() {
                        json_object.get("completed").unwrap().as_boolean().unwrap()
                    } else { 
                        old_todo.completed.clone()
                    }
                };

                let new_order: u64 = {
                    if json_object.get("order").is_some() {
                        json_object.get("order").unwrap().as_u64().unwrap()
                    } else { 
                        old_todo.order.clone()
                    }
                };

                let todo = Todo::new(id.clone(), new_title, new_completed, new_order);
                let todo = self.repository.update(id, todo);

                Ok(Response::with((status::Ok, json::encode(&todo).unwrap())))
            }
            Ok(None) => panic!("No body"),
            Err(err) => panic!("Error: {:?}", err)
        }
    }
}

// == DELETE /todos/:id
pub struct DELETETodoHandler {
    repository: Arc<Repository<Todo>>
}

impl DELETETodoHandler {
    pub fn new(repository: Arc<Repository<Todo>>) -> DELETETodoHandler {
        DELETETodoHandler {
            repository: repository,
        }
    }
}

impl Handler for DELETETodoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let id = String::from(req.extensions.get::<Router>().unwrap().find("id").unwrap());
        self.repository.delete(id);
        Ok(Response::with(status::Ok))
    }
}

