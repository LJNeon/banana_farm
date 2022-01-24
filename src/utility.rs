use std::env::{var, VarError};

use tracing::{subscriber, dispatcher::SetGlobalDefaultError};
use tracing_subscriber::{fmt, layer::SubscriberExt};
use tracing_appender::{non_blocking, rolling};

pub struct Env {
  pub discord_token: String,
  pub pg_info: String
}

impl Env {
  pub fn read_vars() -> Result<Env, VarError> {
    Ok(Env {
      pg_info: format!(
        "host=localhost user={} password={} dbname={}",
        var("PG_USERNAME")?,
        var("PG_PASSWORD")?,
        var("PG_DATABASE")?
      ),
      discord_token: var("DISCORD_TOKEN")?
    })
  }
}

pub fn start_tracing() -> Result<non_blocking::WorkerGuard, SetGlobalDefaultError> {
  let (file_writer, guard) = non_blocking(rolling::daily("./logs", "log"));

  subscriber::set_global_default(tracing_subscriber::fmt()
    .finish()
    .with(fmt::Layer::default().with_writer(file_writer)))?;

  Ok(guard)
}
