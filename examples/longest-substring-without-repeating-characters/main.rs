struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut temp = String::new();
        let mut max_count = 0;

        for ch in s.chars() {
            if temp.contains(ch) {
                let temp_count = temp.chars().count();
                if temp_count > max_count {
                    max_count = temp_count;
                }
                temp = String::from(ch);
            } else {
                temp.push(ch);
            }
        }

        let temp_count = temp.chars().count();
        if temp_count > max_count {
            max_count = temp_count;
        }

        return max_count as i32;
    }
}

fn main() {
    let s = "pwwkew";
    println!("{}", Solution::length_of_longest_substring(s.to_string()));
}