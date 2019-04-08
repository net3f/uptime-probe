use std::error::Error;
pub use config::Config;

mod config;


pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}