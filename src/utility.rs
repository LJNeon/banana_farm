// Banana Farm - An entertainment Discord bot based on Bloons TD 6.
// Copyright (C) 2022 LJ Talbot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env::{var, VarError};

use tracing::{dispatcher::SetGlobalDefaultError, subscriber};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt};
use twilight_model::gateway::{
  payload::outgoing::update_presence::{UpdatePresenceError, UpdatePresencePayload},
  presence::{ActivityType, MinimalActivity, Status}
};
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

  subscriber::set_global_default(
    tracing_subscriber::fmt()
      .finish()
      .with(fmt::Layer::default().with_ansi(false).with_writer(file_writer))
  )?;

  Ok(guard)
}

pub fn build_presence(
  kind: ActivityType, name: String
) -> Result<UpdatePresencePayload, UpdatePresenceError> {
  UpdatePresencePayload::new(
    vec![MinimalActivity { kind, name, url: None }.into()],
    false,
    None,
    Status::Online
  )
}
