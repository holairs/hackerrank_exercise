/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let (hour, mins_secs, am_pm) = (&s[..2], &s[3..8], &s[8..]);

    let hour_24 = match am_pm {
        "PM" if hour != "12" => format!("{:02}", hour.parse::<u8>().unwrap() + 12),
        "AM" if hour == "12" => "00".to_string(),
        _ => hour.to_string(),
    };

    format!("{}:{}", hour_24, mins_secs)
}

fn main() {
    let hour = "12:05:45AM";
    println!("{}", time_conversion(&hour));
}
