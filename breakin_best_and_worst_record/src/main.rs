/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max_count = 0;
    let mut min_count = 0;
    let mut actual_min = scores[0];
    let mut actual_max = scores[0];

    for &score in scores.iter().skip(1) {
        if score > actual_max {
            max_count += 1;
            actual_max = score
        }
        if score < actual_min {
            min_count += 1;
            actual_min = score
        }
    }
    vec![max_count, min_count]
}

fn main() {
    let games: [i32; 9] = [10, 5, 20, 20, 4, 5, 2, 25, 1];
    breaking_records(&games);
}
