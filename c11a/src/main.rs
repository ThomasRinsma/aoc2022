#[derive(Clone,Debug)]
enum Operation {
    Sum(u32),
    Product(u32),
    Square
}

#[derive(Clone,Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test: u32,
    true_monkey: usize,
    false_monkey: usize,
}

use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let input = include_str!("in.txt");

    let re = Regex::new(r"Monkey (.+):
  Starting items: (.*)
  Operation: new = old (.) (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"
    ).unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();

    // Parse the input into Monkey objects
    for cap in re.captures_iter(input) {

        let items = &cap[2]
            .split(", ")
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect::<Vec<u32>>();
            
        let operation = match &cap[3] {
            "+" => Operation::Sum(cap[4].parse().unwrap()),
            "*" => match &cap[4] {
                "old" => Operation::Square,
                _ => Operation::Product(cap[4].parse().unwrap())
            }
            _ => panic!()
        };

        // Assume monkeys are ordered by monkey number
        monkeys.push(
            Monkey {
                items: items.clone().into(),
                operation,
                test: cap[5].parse().unwrap(),
                true_monkey: cap[6].parse().unwrap(),
                false_monkey: cap[7].parse().unwrap(),
            }
        );
    }

    let mut nr_inspected_items = vec![0; monkeys.len()];
    
    // Evaluate the monkey game
    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while !monkeys[monkey_idx].items.is_empty() {
                nr_inspected_items[monkey_idx] += 1;

                let item = monkeys[monkey_idx].items.pop_front().unwrap();
                let mut new = match monkeys[monkey_idx].operation {
                    Operation::Sum(rval) => item + rval,
                    Operation::Product(rval) => item * rval,
                    Operation::Square => item * item
                };

                new = new / 3;

                let other_mon_idx = match new % monkeys[monkey_idx].test == 0  {
                    true => monkeys[monkey_idx].true_monkey,
                    false => monkeys[monkey_idx].false_monkey
                };
                monkeys[other_mon_idx].items.push_back(new);
            }
        }
    }

    // Output is the product of the two highest values
    nr_inspected_items.sort();
    let result: u32 = nr_inspected_items.iter().rev().take(2).product();
    
    println!("result: {}", result);
}
