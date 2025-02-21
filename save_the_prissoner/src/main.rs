fn saveThePrisoner(n: i32, m: i32, s: i32) -> i32 {
    let mut result = (m + s - 1) % n;
    if result == 0 {
        result = n;
    }
    result
}

fn main() {
    let n = 7;
    let m = 19;
    let s = 2;
    let result = saveThePrisoner(n, m, s);
    println!("{}", result);
}
