struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix[0].len();

        for i in 0..((size as f64 / 2_f64).round() as usize) {
            if i == size - i - 1 {
                break;
            }

            for j in i..(size - i - 1) {
                // cache top value
                let temp = matrix[i][j];

                // top                            <--                   left
                matrix[i][j]                       = matrix[size - j - 1][i];
                // left                           <--                        bottom
                matrix[size - j - 1][i]            = matrix[size - i - 1][size - j - 1];
                // bottom                         <--              right
                matrix[size - i - 1][size - j - 1] = matrix[j][size - i - 1];
                // right                          <-- top
                matrix[j][size - i - 1]            = temp;
            }
        }
    }
}

fn main() {
    // [15, 13,  2,  5]
    // [14, 12,  4,  1]
    // [6,   3,  8,  9]
    // [16,  7, 10, 11]

    // [15, 13,  2,  5]
    // [14,  4,  8,  1]
    // [12,  3,  6,  9]
    // [16,  7, 10, 11]
    let mut matrix = vec![
        vec![5,1,9,11],
        vec![2,4,8,10],
        vec![13,3,6,7],
        vec![15,14,12,16],
    ];
    // let mut matrix = vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![7, 8, 9],
    // ];
    Solution::rotate(&mut matrix);
    println!("{:#?}", matrix);
}