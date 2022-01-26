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

use std::{error::Error, sync::Arc};

use futures::stream::StreamExt;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::cluster::Cluster;
use twilight_http::Client;
use twilight_model::{
  channel::message::allowed_mentions::AllowedMentions,
  gateway::{presence::ActivityType, Intents}
};

pub mod events;
pub mod utility;

use utility::{build_presence, start_tracing, Env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let _guard = start_tracing()?;
  let env = Env::read_vars()?;

  let http = Client::builder()
    .token(env.discord_token.clone())
    .default_allowed_mentions(AllowedMentions {
      parse: Vec::new(),
      users: Vec::new(),
      roles: Vec::new(),
      replied_user: false
    })
    .build();
  let http = Arc::new(http);

  let intents = Intents::GUILDS | Intents::GUILD_MEMBERS | Intents::GUILD_MESSAGES;
  let (cluster, mut events) = Cluster::builder(env.discord_token.clone(), intents)
    .http_client(http.clone())
    .presence(build_presence(ActivityType::Watching, "you | /help".into())?)
    .build()
    .await?;
  let cluster = Arc::new(cluster);
  let cluster_spawn = Arc::clone(&cluster);

  let resource_types = ResourceType::GUILD
    | ResourceType::CHANNEL
    | ResourceType::ROLE
    | ResourceType::MEMBER
    | ResourceType::USER
    | ResourceType::USER_CURRENT;
  let cache = InMemoryCache::builder().resource_types(resource_types).build();

  tokio::spawn(async move {
    cluster_spawn.up().await;
  });

  while let Some((_, event)) = events.next().await {
    cache.update(&event);
    tokio::spawn(events::handle_event(event, Arc::clone(&http)));
  }

  Ok(())
}
