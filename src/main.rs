use std::error::Error;

use tracing::info;

pub mod utility;

use utility::{start_tracing, Env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let _guard = start_tracing()?;
  let _ = Env::read_vars()?;

  info!("Success!");

  Ok(())
}
