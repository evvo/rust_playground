use std::fmt::{ Formatter, Result, Display };

pub struct Person {
    pub name: &'static str,
    pub age: &'static str
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "The name is: {}. The age is: {}.", self.name, self.age)
    }
}