#[derive(Debug, PartialEq)]
pub enum Node {
    Data(i32, Node),
    Empty,
}
