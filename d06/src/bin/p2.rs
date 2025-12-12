use aoc_runner::generate_main;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::io::BufRead;
use tracing::debug;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map_ok(|l| l.chars().collect::<Vec<_>>())
        .try_collect()?;

    let width = lines[0].len();
    let columns = (0..=width)
        .map(|j| {
            lines
                .iter()
                .map(|line| line.get(j).unwrap_or(&' '))
                .collect::<Vec<_>>()
        })
        .map(|column| {
            let mut n = 0;
            let mut op = None;
            for char in &column {
                if **char == ' ' {
                    continue;
                }
                match char.to_digit(10) {
                    Some(digit) => n = n * 10 + digit as u64,
                    None => op = Some(**char),
                }
            }
            (n, op)
        });

    let mut current_op = None;
    let mut sum = 0;
    let mut tmp_res = None;
    for (n, op_opt) in columns {
        debug!(?sum, ?tmp_res, ?n, ?op_opt, ?current_op);
        if n == 0 {
            current_op = None;
            sum += tmp_res.unwrap_or(0);
            tmp_res = None;
            continue;
        }

        if op_opt.is_some() {
            current_op = op_opt;
        }

        if let Some(op) = current_op {
            match op {
                '+' => tmp_res = Some(tmp_res.unwrap_or(0) + n),
                '*' => tmp_res = Some(tmp_res.unwrap_or(1) * n),
                _ => return Err(eyre!("Invalid operator")),
            }
        }
    }

    sum += tmp_res.unwrap_or(0);

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 3263827);
    }
}
