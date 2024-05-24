// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution;
impl Solution {
    fn is_bad_version(&self, version: i32) -> bool {
        let first_bad_version = 1702766719;
        return version >= first_bad_version;
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut begin = 1;
        let mut end = n;

        while begin <= end {
            let mid = ((begin as i64 + end as i64) / 2) as i32;
            let is_mid_bad = self.is_bad_version(mid);

            if is_mid_bad {
                if end == mid {
                    end -= 1;
                    continue;
                }
                end = mid;
            } else {
                if begin == mid {
                    begin += 1;
                    continue;
                }
                begin = mid;
            }
        }
		return begin;
    }
}

fn main() {
    let s = Solution;
    println!("{}", s.first_bad_version(2126753390))
}