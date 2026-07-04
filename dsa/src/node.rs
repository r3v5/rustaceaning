pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
