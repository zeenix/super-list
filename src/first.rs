#[derive(Debug, PartialEq)]
pub enum Node<T> {
    Data(T, Box<Node<T>>),
    Empty,
}
