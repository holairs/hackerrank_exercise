use std::collections::HashMap;

fn lonelyinteger(a: &[i32]) -> i32 {

    //*a.iter().find(|&&x| a.iter().filter(|&&y| y == x).count() == 1).unwrap_or(&-1)

    let mut counts = HashMap::new();

    for &num in a {
        *counts.entry(num).or_insert(0) += 1;
    }

    let unique = counts
        .iter()
        .find(|&(_, &count)| count == 1)
        .map(|(&num, _)| num);

    unique.unwrap_or(0)
}

fn main() {
    let a = [1, 2, 3, 4, 3, 2, 1];
    let results = lonelyinteger(&a);
    println!("{}", results);
}
