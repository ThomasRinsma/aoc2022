fn main() {
    let input = include_str!("in.txt");

    let alpha: String = ('a'..='z').chain('A'..='Z').collect();
    
    let result: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);

            let badge = a.chars().filter(|x| b.contains(*x) && c.contains(*x)).next().unwrap();

            return alpha.find(badge).unwrap() + 1;
        })
        .sum();

    println!("result: {}", result);

}
