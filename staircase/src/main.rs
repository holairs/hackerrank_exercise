/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */
//                #
//               ##
//              ###
//             ####
//            #####
//           ######

fn staircase(n: i32) {
    for row in 1..=n {
        println!(
            "{}{}",
            " ".repeat((n - row) as usize),
            "#".repeat(row as usize)
        )
    }
}

fn main() {
    staircase(6)
}
