#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}


pub fn create_empty_list() -> List {
    unimplemented!()
}

pub fn create_non_empty_list() -> List {
    unimplemented!()
}

