fn find_zig_zag_sequence(mut arr: Vec<i32>) -> String {
    arr.sort();
    let n = arr.len();
    let mid = (n - 1) / 2;
    arr.swap(mid, n - 1);

    let mut left = mid + 1;
    let mut right = n - 2;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }

    let result = arr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    result
}

fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7];
    let result = find_zig_zag_sequence(values);
    println!("{}", result);
}
