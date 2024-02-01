pub struct MyQueue {
    queue: Vec<i32>,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            queue: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            while let Some(x) = self.queue.pop() {
                self.stack.push(x);
            }
        }
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        if self.stack.is_empty() {
            self.queue[0]
        } else {
            self.stack[self.stack.len() - 1]
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty() && self.stack.is_empty()
    }
}
