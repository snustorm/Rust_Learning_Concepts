use core::num;
use std::io;
use std::io::prelude::*;


mod method;


use method::input::*;
use method::generics::*;
use method::error_handle::*;
use method::vector::*;



struct Person {
    pub name: String,
    pub age: u32,
}

impl Default for Person {
    fn default() -> Self {
        Self { name: "".to_owned(), age: 0}
    }
}

struct User {
    user_id: i32,
    posts: Vec<String>,
}

impl User {
    fn set_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
}


fn add( a: &i32, b: &i32) -> i32 {
    a + b
}

fn add2( a: i32, b: i32) -> i32 {
    a + b
}


fn main() {
  
    create_vector_and_display();

}
 
