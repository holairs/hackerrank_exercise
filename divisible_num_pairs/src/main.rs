/*
 * Complete the 'divisibleSumPairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */

use std::usize;

//let k = 3;
//let ar = [1, 3, 2, 6, 1, 2];
fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;

    for (idx, &num) in ar.iter().enumerate() {
        for j in (idx + 1)..(n as usize) {
            if (num + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let n = 6;
    let k = 3;
    let ar = [1, 3, 2, 6, 1, 2];
    divisible_sum_pairs(n, k, &ar);
}
