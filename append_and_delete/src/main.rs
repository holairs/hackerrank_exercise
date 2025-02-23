#[allow(unused_variables)]
#[allow(non_snake_case)]

fn appendAndDelete(s: &str, t: &str, k: i32) -> String {
    let mut common_length = 0;

    let min_length = s.len().min(t.len());
    for i in 0..min_length {
        if s.chars().nth(i) == t.chars().nth(i) {
            common_length += 1;
        } else {
            break;
        }
    }

    let operations_needed = (s.len() - common_length) + (t.len() - common_length);

    if operations_needed as i32 > k {
        return "No".to_string();
    }

    if (k as usize - operations_needed) % 2 == 0 || k as usize >= (s.len() + t.len()) {
        return "Yes".to_string();
    }

    "No".to_string()
}

fn main() {
    let s = "hackerhappy";
    let t = "hackerrank";
    let k = 9;
    let result = appendAndDelete(s, t, k);
    println!("{}", result);
}
