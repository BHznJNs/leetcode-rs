struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        fn is_row_range_all_1(
            matrix: &Vec<Vec<char>>,
            row: usize,
            start: usize,
            end: usize,
        ) -> bool {
            let target_row = &matrix[row];
            for ch in &target_row[start..end] {
                if *ch == '0' {
                    return false;
                }
            }
            return true;
        }
        fn is_col_range_all_1(
            matrix: &Vec<Vec<char>>,
            col: usize,
            start: usize,
            end: usize,
        ) -> bool {
            for i in start..end {
                if matrix[i][col] == '0' {
                    return false;
                }
            }
            return true;
        }
        fn is_all_1() {
            //
        }

        let mut xl = 0;
        let mut yl = 0;
        let mut xr = matrix[0].len() - 1;
        let mut yr = matrix.len() - 1;

        while xl < xr && yl < yr {
            //
        }

        todo!()
    }
}

fn main() {
    //
}
