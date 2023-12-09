use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/09.txt")?;
    let lines = input.lines().collect_vec();

    let sum = lines
        .iter()
        .map(|line| {
            let mut xs = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();

            let mut lasts = vec![*xs.last().unwrap()];
            loop {
                let mut nx = vec![];
                for (i, j) in xs.into_iter().tuple_windows() {
                    nx.push(j - i);
                }

                if nx.iter().all(|x| *x == 0) {
                    break;
                }

                lasts.push(*nx.last().unwrap());
                xs = nx;
            }

            lasts.into_iter().sum::<i64>()
        })
        .sum::<i64>();

    println!("{sum}");
    Ok(())
}
