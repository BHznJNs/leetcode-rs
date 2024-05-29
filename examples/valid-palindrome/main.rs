struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();

        if s.len() == 0 {
            return true;
        }

        let mut l = 0_usize;
        let mut r = s.chars().count() - 1;
        let bytes = s.as_bytes();
        while l < r {
            let l_ch = bytes[l];
            let r_ch = bytes[r];
            if l_ch != r_ch {
                return false;
            }

            l += 1;
            r -= 1;
        }

        return true;
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")));
}
