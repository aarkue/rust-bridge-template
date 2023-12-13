use serde::{Deserialize, Serialize};

pub fn add_two_f32(a: f32, b: f32) -> f32 {
    return a + b;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub children: Option<Vec<Person>>,
}
impl Person {
    pub fn from_json_str(json: &str) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

pub fn get_greet_string(person: &Person) -> String {
    let num_children = match &person.children {
        Some(x) => x.len(),
        None => 0,
    };
    return format!(
        "Hello {}! You are {} years old and have {} children.",
        person.name, person.age, num_children
    );
}
