use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;
    let res: i32 = input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|s| s.parse())
                .collect::<Result<Vec<i32>, _>>()?;
            let &[l, w, h] = &dims[..] else {
                unreachable!()
            };

            dims.sort();
            let &[a, b, _] = &dims[..] else {
                unreachable!()
            };

            let area = 2 * l * w + 2 * w * h + 2 * h * l;
            let slack = a * b;
            Ok(area + slack)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum();

    println!("{res}");

    Ok(())
}
