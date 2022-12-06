use itertools::Itertools;

fn main() {
    let input = include_str!("in.txt");

    let result = input
        .chars()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(14)
        .filter(|w| w.iter().map(|(_, c)| c).all_unique())
        .map(|w| w.last().unwrap().0)
        .next()
        .unwrap() + 1;
    

    println!("result = {}", result);
}
