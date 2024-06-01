struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        struct BracketStack {
            vec: Vec<char>,
        }
        impl BracketStack {
            pub fn new() -> Self {
                Self { vec: vec![] }
            }
            pub fn top(&self) -> Option<char> {
                self.vec.last().copied()
            }
            pub fn push(&mut self, ch: char) {
                self.vec.push(ch)
            }
            pub fn pop(&mut self) -> Option<char> {
                self.vec.pop()
            }
            pub fn is_empty(&self) -> bool {
                self.vec.is_empty()
            }
        }

        fn bracket_resolve(bracket: char) -> (bool, char) {
            match bracket {
                '(' => (true , ')'),
                ')' => (false, '('),
                '[' => (true , ']'),
                ']' => (false, '['),
                '{' => (true , '}'),
                '}' => (false, '{'),
                _ => unreachable!(),
            }
        }

        let mut stack = BracketStack::new();
        for ch in s.chars() {
            let (is_start_bracket, paired_bracket) = bracket_resolve(ch);
            if is_start_bracket {
                // is '(' '[' '{'
                stack.push(ch);
                continue;
            }

            // is ')' ']' '}'
            if stack.is_empty() || stack.top().is_some_and(|ch| ch != paired_bracket) {
                return false;
            }
            stack.pop();
        }
        return stack.is_empty();
    }
}

fn main() {
    println!("{}", Solution::is_valid(String::from("()")));
    println!("{}", Solution::is_valid(String::from("()[]{}")));
    println!("{}", Solution::is_valid(String::from("(]")));
    println!("{}", Solution::is_valid(String::from("([)]")));
    println!("{}", Solution::is_valid(String::from("{[]}")));
}
