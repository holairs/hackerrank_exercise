/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut ar = ar.to_vec();
    ar.sort();
    let mut total = 0;
    let mut i = 0;

    while i < n {
        let mut count = 1;
        while (i + 1) < n && ar[i as usize] == ar[(i + 1) as usize] {
            count += 1;
            i += 1;
        }
        total += count / 2;
        i += 1;
    }

    println!("{}", total);
    total
}

fn main() {
    let ar = [10, 20, 20, 10, 10, 30, 50, 10, 20];
    let n = 9;
    sock_merchant(n, &ar);
}
