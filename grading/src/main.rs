/*
 * Complete the 'gradingStudents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let result: Vec<i32> = grades
        .into_iter()
        .map(|note| {
            let next_mult = ((note / 5) + 1) * 5;
            if next_mult - note < 3 && note >= &38 {
                next_mult
            } else {
                *note
            }
        })
        .collect();
    result
}

fn main() {
    let grades = [73, 67, 38, 33];
    let resultado = grading_students(&grades);
    println!("{:?}", resultado);
}
