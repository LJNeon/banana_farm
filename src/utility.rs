use std::env::{var, VarError};

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
