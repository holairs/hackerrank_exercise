/*
 * Complete the 'birthday' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY s
 *  2. INTEGER d
 *  3. INTEGER m
 */

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut result = 0;

    for i in 0..=s.len().saturating_sub(m as usize) {
        let sum: i32 = s[i..i + m as usize].iter().sum();

        if sum == d {
            result += 1;
        }
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // <-- clear the screen
    println!("el result es: {}", result);
    result
}

fn main() {
    let segments: [i32; 5] = [1, 2, 1, 3, 2];
    let day = 3;
    let month = 2;
    birthday(&segments, day, month);
}
