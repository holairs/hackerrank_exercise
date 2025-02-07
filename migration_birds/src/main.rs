/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
//1 2 3 4 5 4 3 2 1 3 4
fn migratory_birds(arr: &[i32])  -> i32 {
    let mut max_count = 0;
    let mut max_type = i32::MAX;

    for &item in arr {
        let count = arr.iter().filter(|&&x| x == item).count();

        if count > max_count || (count == max_count && item < max_type) {
            max_count = count;
            max_type = item;
        }
    }
    println!("{}", max_type);
    max_type
}

fn main() {
    let birds = [1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
    migratory_birds(&birds);
}
