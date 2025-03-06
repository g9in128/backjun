pub struct Heap<T: Ord + Clone> {
    heap: Vec<T>,
}

impl<T> Heap<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    pub fn push(value: &T) {
        Heap::push(value);
    }
}
