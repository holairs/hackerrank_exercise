/*
 * Complete the 'hurdleRace' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY height
 */
fn hurdle_race(k: i32, height: &[i32]) -> i32 {
    let max_height = height
        .iter()
        .max() // Encuentra el máximo de los filtrados
        .filter(|&&x| x > 0) // Filtra valores mayores que 0
        .copied() // Convierte &i32 a i32 (importante!)
        .unwrap_or(0);
    max_height
}

fn main() {
    let height = [2, 5, 4, 5, 2];
    let k = 7;
    let result = hurdle_race(k, &height);
    println!("{}", result);
}
