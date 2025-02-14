use std::char;

fn counting_valleys(steps: i32, path: &str) -> i32 {
    let mut position = 0;
    let mut valley = false;
    let mut total = 0;

    for s in path.chars() {
        if s == 'U' {
            position += 1;
        } else {
            position -= 1;
        }

        if position < 0 && !valley {
            valley = true;
        }

        if position == 0 && valley {
            total += 1;
            valley = false;
        }
    }

    total
}

fn main() {
    //let steps = 12;
    //let path = "UDDDUDUU";
    let steps = 12;
    let path = "DDUUDDUDUUUD";
    let result = counting_valleys(steps, &path);
    println!("{}", result);
}
