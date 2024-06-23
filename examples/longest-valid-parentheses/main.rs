struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        fn is_valid_parentheses(s: &str) -> bool {
            let mut counter = 0;
            for ch in s.chars() {
                if ch == '(' {
                    counter += 1;
                    continue;
                }
                if ch == ')' {
                    if counter == 0 {
                        return false;
                    }
                    counter -= 1;
                }
            }
            return counter == 0;
        }

        if s.is_empty() || s.len() == 1 {
            return 0;
        }

        let mut paren_counter = 0;
        let mut pair_counter = 0;
        let mut result = 0;
        for ch in s.chars() {
            if ch == '(' {
                paren_counter += 1;
                continue;
            }
            if ch == ')' {
                if paren_counter == 0 {
                    result = result.max(pair_counter * 2);
                    pair_counter = 0;
                    continue;
                }
                paren_counter -= 1;
                pair_counter += 1;
            }
        }
        result = result.max(pair_counter * 2);

        return result;
    }
}

fn main() {
    // println!("{}", Solution::longest_valid_parentheses(String::from("(()")));
    // println!("{}", Solution::longest_valid_parentheses(String::from(")()())")));
    // println!("{}", Solution::longest_valid_parentheses(String::from("()(()")));
    println!("{}", Solution::longest_valid_parentheses(String::from("))))())()()(()")));
}
