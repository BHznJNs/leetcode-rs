use std::collections::VecDeque;

struct MonoStack {
    list: VecDeque<i32>
}
impl MonoStack {
    pub fn new() -> Self {
        Self {
            list: VecDeque::new()
        }
    }
    pub fn push(&mut self, val: i32) {
        if self.list.is_empty() || self.top().is_some_and(|top| top > val) {
            self.list.push_front(val);
            return;
        }

        let mut insert_index = self.list.len();
        for (index, item) in self.list.iter().enumerate() {
            if *item > val {
                insert_index = index;
                break;
            }
        }
        self.list.insert(insert_index, val);
    }
    pub fn remove(&mut self, val: i32) {
        let Ok(index) = self.list.binary_search(&val) else {
            unreachable!()
        };
        self.list.remove(index);
    }
    pub fn top(&self) -> Option<i32> {
        self.list.front().copied()
    }
}

struct MinStack {
    list: Vec<i32>,
    mono: MonoStack,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            list: vec![],
            mono: MonoStack::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.list.push(val);
        self.mono.push(val);
    }

    fn pop(&mut self) {
        let popped_val = self.list.pop().unwrap();
        self.mono.remove(popped_val);
    }

    fn top(&self) -> i32 {
        self.list.last().copied().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.mono.top().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[test]
fn monostack_test() {
    let mut stack = MonoStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-1);
    dbg!(stack.list);
    // stack.push(3);
    // assert_eq!(stack.top(), Some(3));
    // stack.push(1);
    // assert_eq!(stack.top(), Some(1));
    // stack.push(2);
    // assert_eq!(stack.top(), Some(1));
    // stack.push(10);
    // assert_eq!(stack.top(), Some(1));
}

fn main() {
    let mut stack = MinStack::new();

    // stack.push(-2);
    // stack.push(0);
    // stack.push(-3);
    // assert_eq!(stack.get_min(), -3);
    // stack.pop();
    // assert_eq!(stack.top(), 0);
    // assert_eq!(stack.get_min(), -2);

    stack.push(-2);
    stack.push(0);
    stack.push(-1);
    assert_eq!(stack.get_min(), -2);
    assert_eq!(stack.top(), -1);
    stack.pop();
    assert_eq!(stack.get_min(), -2);
}
