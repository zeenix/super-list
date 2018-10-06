pub mod first;

#[cfg(test)]
mod tests {
    use ::first::Node;

    #[test]
    fn it_works() {
        let l = Node::Empty::<i32>;
        assert_eq!(l, Node::Empty);
    }
}
