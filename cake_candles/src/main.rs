/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    candles.iter().filter(|&x| x == max).count() as i32
}

fn main() {
    let candles = [3, 2, 1, 3];
    println!("{:?}", birthday_cake_candles(&candles))
    
}
