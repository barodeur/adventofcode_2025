use color_eyre::Result;
use std::io::{stdin, BufRead};

pub use color_eyre;
pub use tracing_subscriber;

pub use aoc_runner_macro::resolver;

pub fn run<F, T>(solve: F) -> Result<()>
where
    F: FnOnce(Box<dyn BufRead>) -> Result<T>,
    T: std::fmt::Display,
{
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let reader = Box::new(stdin().lock());
    let result = solve(reader)?;
    println!("{}", result);

    Ok(())
}
