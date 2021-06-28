use std::str::FromStr;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        /* TODO */
    }
}