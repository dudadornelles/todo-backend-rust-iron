
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Todo {
    pub id: String,
    pub title: String
}

impl Todo {
    pub fn new(id: String, title: String) -> Todo {
        Todo {
            id: id,
            title: title
        }
    }
}



