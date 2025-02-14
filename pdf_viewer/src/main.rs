/*
 * Complete the 'designerPdfViewer' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY h
 *  2. STRING word
 */

use std::char;

fn designer_pdf_viewer(h: &[i32], word: &str) -> i32 {
    let mut values: Vec<i32> = Vec::new();
    let w: Vec<char> = word.chars().collect();
    let a = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    for i in w {
        for l in 0..26 {
            if i.to_string() == a[l] {
                values.push(h[l] as i32);
            }
        }
    }
    let result = values.iter().max().unwrap() * word.len() as i32;
    result
}

fn main() {
    let h = [
        1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7,
    ];
    let word = "zaba";
    let result = designer_pdf_viewer(&h, word);
    println!("{}", result);
}
