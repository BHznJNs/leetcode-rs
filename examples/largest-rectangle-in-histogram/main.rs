struct Solution;

struct Stack {
    list: Vec<usize>
}
impl Stack {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
    pub fn push(&mut self, val: usize) {
        self.list.push(val)
    }
    pub fn pop(&mut self) -> Option<usize> {
        self.list.pop()
    }
    pub fn top(&self) -> Option<usize> {
        self.list.last().copied()
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack = Stack::new();
        let height_len = heights.len();
        
        for i in 0..=heights.len() {
            let height = if i == height_len { 0 } else { heights[i] };
            if stack.is_empty() || stack.top().is_some_and(|i| height >= heights[i]) {
                stack.push(i);
            } else {
                let popped = stack.pop().unwrap();
                let width = if stack.is_empty() {
                    i
                } else {
                    i - 1 - stack.top().unwrap()
                };
                let area = heights[popped] * width as i32;
                max_area = max(max_area, area);
            }
        }

        return max_area;
    }
}

fn main() {
    println!("{}", Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
}
