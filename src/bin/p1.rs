use color_eyre::Result;
use d01::rotation::Rotation;
use std::io::stdin;
use tracing::debug;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let rotations = Rotation::iter_from(stdin().lock());

    let mut position = 50;
    let mut count = 0;

    for rotation in rotations {
        let rotation = rotation?;
        debug!(?position, ?rotation);

        position = (100 + position + (rotation.angle() % 100)) % 100;

        if position == 0 {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
