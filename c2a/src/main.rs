fn main() {
    let input = include_str!("in.txt");

    let result: usize = input
        .lines()
        .map(|line| {
            let opp = "ABC".find(line.chars().nth(0).unwrap()).unwrap();
            let our = "XYZ".find(line.chars().nth(2).unwrap()).unwrap();

            if opp == our {
                return our + 1 + 3; // draw
            }
            else if (opp + 1) % 3 == our {
                return our + 1 + 6; // win
            }
            else {
                return our + 1; // loose
            }
        })
        .sum();
    
    println!("result: {}", result);

}
