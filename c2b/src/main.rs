fn main() {
    let input = include_str!("in.txt");
    
    let result: usize = input
        .lines()
        .map(|line| {
            let opp = "ABC".find(line.chars().nth(0).unwrap()).unwrap();
            let goal = line.chars().nth(2).unwrap();

            if goal == 'Y' {
                // draw: choose opponent's move
                return 3 + 1 + opp;
            }
            else if goal == 'X' {
                // lose: choose opponent's move minus one (in mod 3 that's plus 2)
                return 0 + 1 + (opp + 2) % 3;
            }
            else {
                // win: choose opponent's move plus one
                return 6 + 1 + (opp + 1) % 3;
            }
        })
        .sum();
    println!("result: {}", result);

}
