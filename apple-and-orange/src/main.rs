/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s            House start
 *  2. INTEGER t            House end
 *  3. INTEGER a            Apple tree start
 *  4. INTEGER b            Orange tree start
 *  5. INTEGER_ARRAY apples Amount and apples movement
 *  6. INTEGER_ARRAY orangesAmount and oranges movement
 */

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples = apples.iter().filter(|&&x| x + a >= s && x + a <= t).count();
    let oranges = oranges.iter().filter(|&&x| x + b >= s && x + b <= t).count();
    println!("{:?}", apples);
    println!("{:?}", oranges)
}

fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples: [i32; 3] = [-2, 2, 1];
    let oranges: [i32; 2] = [5, -6];
    print!("\x1B[2J\x1B[1;1H");
    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
