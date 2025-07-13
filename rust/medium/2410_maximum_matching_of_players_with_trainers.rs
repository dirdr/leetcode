impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut trainers = trainers;
        let mut players = players;
        trainers.sort_unstable_by(|a, b| b.cmp(&a));
        players.sort_unstable_by(|a, b| b.cmp(&a));
        let mut answer = 0;
        let (mut p, mut t) = (0, 0);
        while p < players.len() && t < trainers.len() {
            if trainers[t] >= players[p] {
                answer += 1;
                t += 1;
            }
            p += 1;
        }
        answer
    }
}
