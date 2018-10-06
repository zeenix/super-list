#[derive(Debug, PartialEq)]
pub enum Node {
    Data(i32, Box<Node>),
    Empty,
}
