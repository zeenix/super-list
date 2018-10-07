pub mod first;

#[cfg(test)]
mod tests {
    use ::first::Node;

    #[test]
    fn it_works() {
        let next = Box::new(Node::Empty::<i32>);
        let l = Node::Data(42, next);
        match l {
            Node::Data(d, n) => {
                assert_eq!(d, 42);
                assert_eq!(n, Box::new(Node::Empty::<i32>));
            },
            Node::Empty => panic!(),
        }
    }
}
