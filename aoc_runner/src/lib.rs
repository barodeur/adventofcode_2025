use color_eyre::{self, Result};

pub fn init() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    Ok(())
}

/// Generates a main function that calls the provided solve function.
#[macro_export]
macro_rules! generate_main {
    ($solve:ident) => {
        fn main() -> Result<()> {
            $crate::init()?;

            let reader = std::io::stdin().lock();
            let result = $solve(reader)?;
            println!("{}", result);

            Ok(())
        }
    };
}
