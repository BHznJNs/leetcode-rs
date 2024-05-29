struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        const A_USIZE: usize = 'a' as usize;
        let mut bucket1 = [0; 26];
        let mut bucket2 = [0; 26];
        for ch in s.chars() {
            bucket1[ch as usize - A_USIZE] += 1;
        }
        for ch in t.chars() {
            bucket2[ch as usize - A_USIZE] += 1;
        }
        return bucket1.eq(&bucket2);
    }
}

fn main() {}
