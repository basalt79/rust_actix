use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
  first_name: String,
  last_name: String,
  the_number: i32,
}

impl Person {
  pub fn new(first: &str, last: &str, number: i32) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
      the_number: number,
    }
  }
}
