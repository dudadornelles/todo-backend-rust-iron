
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub url: String
}

impl Todo {

    pub fn new(id: String, title: String, completed: bool) -> Todo {
        let url = String::from("http://localhost:3000/todos/") + &id;
        Todo {
            id: id,
            title: title,
            completed: completed,
            url: url
        }

    }
}



