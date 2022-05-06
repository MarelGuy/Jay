pub(crate) mod node {
    pub struct Node<T> {
        children: Vec<T>,
        node: T,
    }

    impl<T> Node<T> {
        pub fn new(children: Vec<T>, node: T) -> Self {
            Self { children, node }
        }
    }
}
pub(crate) mod declarations;
pub(crate) mod math_ops;
pub(crate) mod types;
