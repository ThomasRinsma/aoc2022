fn main() {
    let input = include_str!("in.txt");

    let mut elf_totals: Vec<i64> = input
        .split("\n\n")
        .map(|s| s
            .lines()
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .sum::<i64>()
        )
        .collect::<Vec<i64>>();

    elf_totals.sort();

    let sum: i64 = elf_totals.iter().rev().take(3).sum();

    println!("{}", sum);
}
