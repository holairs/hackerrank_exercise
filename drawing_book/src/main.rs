use std::cmp::min;

fn page_count(n: i32, p: i32) -> i32 {
    let mut skip = 0;
    let mut result = 0;

    if p <= (n / 2) {
        for page in 1..=n {
            if page % 2 == 0 {
                skip += 1;
            }
            if page == p {
                result = skip
            }
        }
    } else {
        for page in (1..=n).rev() {
            if page % 2 != 0 && page != n {
                skip += 1;
            }
            if page == p {
                result = skip
            }
        }
    }
    println!("{}", res);
    result
}

fn main() {
    let n = 15603;
    let p = 6957;
    let result = page_count(n, p);
    println!("{}", result);
}
