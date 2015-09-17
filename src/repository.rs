use std::thread;
use std::sync::Arc;

use ::todo::Todo;

#[test]
fn should_add_many_values_with_threds_with_an_id() {
    let repo: Arc<Repository<String>> = Arc::new(Repository::new());

    let handles: Vec<_> = vec![0,1,2].into_iter().map(|i| {
        let repo = repo.clone();
        return thread::spawn(move|| repo.add(i.to_string(), String::from("abc")));
    }).collect();

    for h in handles { h.join().unwrap(); }

    
    let actual = repo.get("0".to_string());
    assert_eq!(actual, "abc");
}

#[test]
fn should_work_with_a_complex_struct() {
    let repo: Arc<Repository<Todo>> = Arc::new(Repository::new());

    let handles: Vec<_> = vec![0,1,2].into_iter().map(|i| {
        let repo = repo.clone();
        let t = Todo::new(i.to_string(), String::from("O gosh..."));
        return thread::spawn(move|| repo.add(i.to_string(), t));
    }).collect();

    for h in handles { h.join().unwrap(); }

    
    let actual = repo.get("0".to_string());
    assert_eq!(actual.title, String::from("O gosh..."));
}

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Repository<T> where T: Clone {
   entities: Mutex<HashMap<String, T>>
}

impl<T> Repository<T> where T: Clone {
    pub fn new() -> Repository<T> {
        Repository {
            entities: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, key: String, value: T) {
        let mut m_entities = self.entities.lock().unwrap();
        m_entities.insert(key, value);
    }

    pub fn get(&self, key: String) -> T {
        let m_entities = self.entities.lock().unwrap();
        let value = m_entities.get(&key).unwrap();
        (*value).clone()
    }
}
