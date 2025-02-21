

fn viral_advertising(n: i32) -> i32 {
    let mut shared = 5;
    let mut cumulative = 0;
    for _ in 1..=n {
        let liked = shared / 2;
        cumulative += liked;
        shared = liked * 3;
    }

    cumulative
}

fn main() {
    let n = 5;
    println!("{}", viral_advertising(n));
}
