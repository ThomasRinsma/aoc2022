use itertools::Itertools;

fn main() {
    let input = include_str!("in.txt");

    let win_size = 14;

    let result = input
        .chars()
        .collect::<Vec<_>>()
        .windows(win_size)
        .enumerate()
        .filter(|(_, w)| w.iter().all_unique())
        .map(|(i, _)| i + win_size)
        .next()
        .unwrap();
    

    println!("result = {}", result);
}
