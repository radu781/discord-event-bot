use crate::messages::torrent::TorrentMessage;
use clap::Subcommand;
use serenity::all::{ChannelId, CreateMessage};
use std::env;

#[derive(Subcommand, Debug)]
pub(crate) enum MessageType {
    Torrent(TorrentMessage),
}

impl MessageType {
    pub(crate) async fn message(&self) -> CreateMessage {
        match self {
            MessageType::Torrent(m) => m.message().await,
        }
    }

    pub(crate) fn channel_id(&self) -> ChannelId {
        match self {
            MessageType::Torrent(_) => ChannelId::new(
                env::var("torrents_channel")
                    .expect("\"torrents_channel\" not found in environment")
                    .parse::<u64>()
                    .expect("Could not parse \"torrents_channel\""),
            ),
        }
    }
}
