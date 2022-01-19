use std::error::Error;
use tracing::info;

pub mod utility;

use utility::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  tracing_subscriber::fmt::init();

  let _ = Env::read_vars()?;

  info!("Success!");

  Ok(())
}
