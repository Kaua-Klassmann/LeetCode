use std::collections::VecDeque;

struct MyQueue {
    queue: VecDeque<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);

    assert_eq!(obj.peek(), 1);
    assert_eq!(obj.pop(), 1);
    assert_eq!(obj.empty(), false);
}
