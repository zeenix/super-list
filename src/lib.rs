pub mod first;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let l = ::first::Node::Empty::<i32>;
        assert_eq!(l, ::first::Node::Empty);
    }
}
