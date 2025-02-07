/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let excluded_value: i32 = bill.iter().sum::<i32>() - bill.get(k as usize).copied().unwrap_or(0);

    if (excluded_value / 2) + b == excluded_value {
        println!("Bon Appetit");
    } else {
        println!("{}", b - (excluded_value / 2));
    }
}

fn main() {
    let bill = [3, 10, 2, 9];
    let k = 1;
    let b = 12;
    bon_appetit(&bill, k, b);
}
