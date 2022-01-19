use std::env::{var, VarError};

pub struct Env {
  pub pg_info: String,
  pub discord_token: String
}

impl Env {
  pub fn read_vars() -> Result<Env, VarError> {
    Ok(Env {
      pg_info: format!("host=localhost user={} password={} dbname={}", var("PG_USER")?, var("PG_PASSWORD")?, var("PG_DATABASE")?),
      discord_token: var("DISCORD_TOKEN")?
    })
  }
}
