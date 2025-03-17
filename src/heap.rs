pub struct Heap {
    heap: Vec<i32>,
}

impl Heap {
    pub fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        let mut now = self.size();
        self.heap.push(value);
        if now == 0 {
            return;
        }
        while now > 0 {
            let head = (now - 1) / 2;
            if self.heap[now] < self.heap[head] {
                self.swap(now, head);
                now = head;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }
        let ret: Option<i32> = Some(self.heap.swap_remove(0));

        let mut i = 0;
        while i < self.heap.len() {
            let (left, right) = (i * 2 + 1, i * 2 + 2);
            let data_i = self.heap[i];
            let bigger;
            if left >= self.size() {
                break;
            } else if right >= self.size() {
                bigger = left;
            } else {
                let (data_left, data_right) = (self.heap[left], self.heap[right]);
                if data_left < data_right {
                    bigger = left;
                } else {
                    bigger = right;
                }
            }
            if data_i > self.heap[bigger] {
                self.swap(i, bigger);
                i = bigger;
            } else {
                break;
            }
        }
        ret
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self.heap[a];
        self.heap[a] = self.heap[b];
        self.heap[b] = tmp;
    }
}
