fn main() {
    let input = include_str!("in.txt");

    let alpha: String = ('a'..='z').chain('A'..='Z').collect();
    
    let result: usize = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            let c: char = left.chars().filter(|c| right.contains(*c)).next().unwrap();

            return alpha.find(c).unwrap() + 1;
        })
        .sum();

    println!("result: {}", result);

}
