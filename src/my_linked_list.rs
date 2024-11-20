
#[derive(Debug)]
pub struct Node<T> {
    pub data:T,
    pub next_body:Option<Box<Node<T>>>
}

impl<T:Clone> Node<T> {
    pub fn new(data:T) -> Self {
        Node{data,next_body: None}
    }
    
}

#[derive(Debug)]
pub struct MyLinkedList<T> {
    pub head : Option<Box<Node<T>>>,
}

impl<T> MyLinkedList<T> {
    
    pub fn new() -> Self {
        MyLinkedList{head : None}
    }
}