static N: u128 = 2;
static M: u128 = 3;
static K: u128 = 5;

// fn dp(build_cost: u128) -> u128 {
//     if build_cost == 2 {
//         return M * M * dp(2 + 1);
//     }
//     if build_cost == K {
//         if K == N * M {
//             return 1;
//         }
//         return N * M - build_cost
//     }

//     (N * M - build_cost) * dp(build_cost + 1)
// }
fn dp(build_cost: u128) -> u128 {
    if build_cost == 2 {
        return M * M;
    }
    // 
    todo!()
}


fn main() {
    println!("{}", dp(K) % (10_u128.pow(9) + 7));
    // println!("{}", dp(2) % (10_u128.pow(9) + 7));
}
