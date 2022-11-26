use std::*;

pub trait do_something {
    fn something(&self) -> String;

    fn do_default(&self) -> String {
        format!("{} have done.", self.something())
    }
}

pub struct Human {
    pub todo_list: Vec<String>,
}

impl do_something for Human {
    fn something(&self) -> String {
        format!("{:?}", self.todo_list)
    }
}

fn main() {

    let mut human = Human {
        todo_list: vec!["eat".to_string(),"sleep".to_string(),"play".to_string()],
    };

    println!("{}", human.something());
    println!("{}", human.do_default());
    
}
