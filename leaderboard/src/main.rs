/*
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut leaderboard: Vec<i32> = ranked.to_vec();
    leaderboard.dedup();
    let n = leaderboard.len();
    let mut result = Vec::with_capacity(player.len());

    for score in player {
        let mut low = 0;
        let mut high = n;

        while low < high {
            let mid = (low + high) / 2;
            if score >= &leaderboard[mid] {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        result.push((low + 1) as i32);
    }
    result
}

fn main() {
    let ranked = [100, 100, 50, 40, 40, 20, 10];
    let player = [5, 25, 50, 120];
    let result = climbingLeaderboard(&ranked, &player);
    println!("{:?}", result);
}
