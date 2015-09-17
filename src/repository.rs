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

#[test]
fn should_return_all() {
    let repo: Arc<Repository<u32>> = Arc::new(Repository::new());

    repo.add(0.to_string(), 10);
    repo.add(1.to_string(), 20);

    let all: Vec<u32> = repo.all();

    assert_eq!(all[0], 10);
    assert_eq!(all[1], 20);
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

    pub fn add(&self, key: String, value: T) -> T where T: Clone {
        let mut m_entities = self.entities.lock().unwrap();
        m_entities.insert(key, value.clone());
        value
    }

    pub fn get(&self, key: String) -> T {
        let m_entities = self.entities.lock().unwrap();
        let value = m_entities.get(&key).unwrap();
        (*value).clone()
    }

    pub fn all(&self) -> Vec<T> {
        let m_entities = self.entities.lock().unwrap();
        let mut values = Vec::new();
        for (_, v) in m_entities.iter() {
            values.push((*v).clone());
        }
        values
    }
}
