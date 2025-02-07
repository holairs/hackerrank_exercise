/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn day_of_programmer(year: i32) -> String {
    if year == 1918 {
        return "26.09.1918".to_string();
    }

    if (year <= 1917 && year % 4 == 0)
        || (year > 1918 && (year % 400 == 0 || year % 4 == 0 && year % 100 != 0))
    {
        return format!("12.09.{}", year);
    } else {
        return format!("13.09.{}", year);
    }
}

fn main() {
    let year = 1800;
    let result = day_of_programmer(year);
    println!("{}", result);
}
