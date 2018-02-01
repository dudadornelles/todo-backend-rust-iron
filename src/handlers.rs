extern crate bodyparser;

use std::sync::Arc;

use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use router::Router;
use uuid::Uuid;

use repository::Repository;
use todo::Todo;

macro_rules! handler {
    ($x:ident) => {
        pub struct $x {
            repository: Arc<Repository<Todo>>,
        }

        impl $x {
            pub fn new(repository: Arc<Repository<Todo>>) -> $x {
                $x { repository: repository }
            }
        }
    }
}

// == GET /todos
handler!(GETTodosHandler);
impl Handler for GETTodosHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let all = self.repository.all();
        let all_json = json::encode(&all).unwrap();
        Ok(Response::with((status::Ok, all_json)))
    }
}

// == POST /todos
handler!(POSTTodosHandler);
impl Handler for POSTTodosHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        return match json_body {
            Ok(Some(json_body)) => {
                let id = Uuid::new_v4().hyphenated().to_string();
                let json_object = json_body.as_object().unwrap();
                
                let new_title: String = { 
                    if json_object.get("title").is_some() {
                        String::from(json_object.get("title").unwrap().as_str().unwrap())
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
handler!(DELETETodosHandler);
impl Handler for DELETETodosHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.repository.delete_all();
        Ok(Response::with(status::Ok))
    }
}

// == GET /todos/:id
handler!(GETTodoHandler);
impl Handler for GETTodoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let todo = self.repository.get(String::from(id));
        Ok(Response::with((status::Ok, json::encode(&todo).unwrap())))
    }
}

// == PATCH /todos/:id
handler!(PATCHTodoHandler);
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
                        String::from(json_object.get("title").unwrap().as_str().unwrap())
                    } else { 
                        old_todo.title.clone() 
                    }
                };

                let new_completed: bool = {
                    if json_object.get("completed").is_some() {
                        json_object.get("completed").unwrap().as_bool().unwrap()
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
handler!(DELETETodoHandler);
impl Handler for DELETETodoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let id = String::from(req.extensions.get::<Router>().unwrap().find("id").unwrap());
        self.repository.delete(id);
        Ok(Response::with(status::Ok))
    }
}
