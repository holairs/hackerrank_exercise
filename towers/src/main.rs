fn towers(n: i32, m: i32) -> i32 {
    if m == 1 || n % 2 == 0 {
        return 2;
    }
    1
}

fn main() {
    let n = 3;
    let m = 6;
    let result = towers(n, m);
    println!("{}", result);
}
