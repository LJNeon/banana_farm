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

use tracing::{info, warn};
use twilight_gateway::Event;
use twilight_http::Client;

pub async fn handle_event(
  event: Event, _: Arc<Client>
) -> Result<(), Box<dyn Error + Send + Sync>> {
  match event {
    Event::ShardConnected(info) => {
      info!("Shard {} connected.", info.shard_id);
    },
    Event::ShardDisconnected(info) => {
      warn!(
        "Shard {} disconnected:{}{}",
        info.shard_id,
        info.code.map(|x| format!(" [{}]", x)).unwrap_or_default(),
        info.reason.as_ref().map(|x| format!(" {}", x)).unwrap_or_default()
      );
    },
    _ => {}
  }

  Ok(())
}
