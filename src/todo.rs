
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub order: u64,
    pub completed: bool,
    pub url: String
}

impl Todo {
    pub fn new(id: String, title: String, completed: bool, order: u64) -> Todo {
        let url = String::from("https://floating-hamlet-5988.herokuapp.com/todos/") + &id;
    //    let url = String::from("http://0.0.0.0:8080/todos/") + &id;
        Todo {
            id: id,
            title: title,
            order: order,
            completed: completed,
            url: url
        }
    }
}

