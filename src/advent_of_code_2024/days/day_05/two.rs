#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &String) -> u32 {
    let mut input_data = _input.split("\n\n");
    let mut rule_data = HashMap::<u32,HashSet<u32>>::new();

    input_data.next().unwrap()
        .split("\n")
        .map(|line| line.split("|"))
        .map(|mut line| (
            line.next().unwrap().parse::<u32>().unwrap(),
            line.next().unwrap().parse::<u32>().unwrap()
        )).for_each(|(left, right)| {
            rule_data.entry(left)
                .and_modify(|nums| { nums.insert(right); })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(right);
                    set
                });
        });

    let updates = input_data.next().unwrap()
        .split("\n")
        .map(|line| line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>());

    let mut result = 0_u32;

    for mut update in updates {
        let mut ordered = true;

        for (i, &num) in update.iter().enumerate() {
            for j in 0_usize..i {
                if rule_data.entry(num).or_insert(HashSet::new()).contains(&update[j]) {
                    ordered = false;
                }
            }
        }

        if ordered {
            continue;
        }

        while !ordered {
            ordered = true;

            for (i, &num) in update.clone().iter().enumerate() {
                for j in 0_usize..i {
                    if rule_data.entry(num).or_insert(HashSet::new()).contains(&update[j]) {
                        update.swap(i,j);
                        ordered = false;
                    }
                }
            }
        }

        result += update.get(update.len() / 2).unwrap()
    }

    result
}