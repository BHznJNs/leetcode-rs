struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        fn _is_palindrome(s: &str) -> bool {
            if s.len() == 0 || s.len() == 1 {
                return true;
            }
    
            let mut l = 0;
            let mut r = s.chars().count() - 1;
            let bytes = s.as_bytes();
            while l < r {
                let l_ch = bytes[l];
                let r_ch = bytes[r];
                if l_ch == r_ch {
                    l += 1;
                    r -= 1;
                    continue;
                }
                return false;
            }
            return true;
        }

        let s = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();

        if s.len() == 0 || s.len() == 1 {
            return true;
        }

        let mut l = 0;
        let mut r = s.chars().count() - 1;
        let bytes = s.as_bytes();
        while l < r {
            let l_ch = bytes[l];
            let r_ch = bytes[r];
            if l_ch == r_ch {
                l += 1;
                r -= 1;
                continue;
            }

            let sub_str1 = &s[(l+1)..=r];
            let sub_str2 = &s[l..=(r-1)];
            return _is_palindrome(sub_str1) || _is_palindrome(sub_str2);
        }

        return true;
    }
}

fn main() {
    // 19: 'c'
    // 20: 'u'
    // 21: 'p'
    // 78: 'p'
    // 79: 'u'
    // 80: 'c'
    // 81: 'u'
    println!("{}", Solution::valid_palindrome(String::from("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga")));
    // println!("{}", Solution::is_palindrome(String::from("mgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga")));
}
