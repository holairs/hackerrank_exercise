/*
 * Complete the 'beautifulDays' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER i
 *  2. INTEGER j
 *  3. INTEGER k
 */

fn beautiful_days(i: i32, j: i32, k: i32) -> i32 {
    let mut total = 0;
    for i in i..=j {
        let rev = i
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let result = i - rev;
        if result % k == 0 {
            total += 1;
        }
    }
    total
}

fn main() {
    let i = 20;
    let j = 23;
    let k = 6;
    let result = beautiful_days(i, j, k);
    println!("{}", result);
}
