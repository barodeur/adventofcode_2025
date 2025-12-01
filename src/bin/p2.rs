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
        let new_position = (100 + position + (rotation.angle() % 100)) % 100;

        let full_rotations_count = rotation.angle().abs() / 100;

        let goes_over_zero = match rotation {
            Rotation::Right(_) if position != 0 && new_position != 0 && new_position < position => {
                true
            }
            Rotation::Left(_) if position != 0 && new_position != 0 && new_position > position => {
                true
            }
            _ => false,
        };

        count += full_rotations_count;
        if goes_over_zero {
            count += 1;
        }
        if new_position == 0 {
            count += 1;
        }

        debug!(
            ?position,
            ?rotation,
            ?new_position,
            ?full_rotations_count,
            ?goes_over_zero,
            ?count
        );

        position = new_position;
    }

    println!("{}", count);

    Ok(())
}
