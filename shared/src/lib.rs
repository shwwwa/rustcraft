use std::time::Duration;

use bevy::prelude::Resource;
use bevy_renet::renet::{ChannelConfig, ConnectionConfig, SendType};
use bincode::Options;

pub mod constants;
pub mod messages;
pub mod players;
pub mod world;

pub use constants::*;

#[derive(Resource, Debug, Clone)]
pub struct GameFolderPaths {
    pub game_folder_path: String,
    pub assets_folder_path: String,
}

#[derive(Resource, Debug, Clone)]
pub struct SpecialFlag {
    pub special_flag: bool,
}

#[derive(Resource)]
pub struct GameServerConfig {
    pub world_name: String,
    pub is_solo: bool,
}

fn get_customized_default_channels() -> Vec<ChannelConfig> {
    let memory = 128 * 1024 * 1024;
    vec![
        ChannelConfig {
            channel_id: 0,
            max_memory_usage_bytes: memory,
            send_type: SendType::Unreliable,
        },
        ChannelConfig {
            channel_id: 1,
            max_memory_usage_bytes: memory,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_millis(300),
            },
        },
        ChannelConfig {
            channel_id: 2,
            max_memory_usage_bytes: memory,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_millis(300),
            },
        },
    ]
}

pub fn get_shared_renet_config() -> ConnectionConfig {
    ConnectionConfig {
        client_channels_config: get_customized_default_channels(),
        server_channels_config: get_customized_default_channels(),
        ..Default::default()
    }
}

pub fn game_message_to_payload<T: serde::Serialize>(message: T) -> Vec<u8> {
    bincode::options().serialize(&message).unwrap()
    // let payload = bincode::options().serialize(&message).unwrap();
    // payload
    // lz4::block::compress(&payload, None, false).unwrap()
}

pub fn payload_to_game_message<T: serde::de::DeserializeOwned>(
    payload: &[u8],
) -> Result<T, bincode::Error> {
    // let decompressed_payload = lz4::block::decompress(payload, None).unwrap();
    // bincode::options().deserialize(&decompressed_payload)
    bincode::options().deserialize(payload)
}
