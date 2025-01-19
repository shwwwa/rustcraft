use bevy::{math::IVec3, prelude::ResMut};
use bevy_renet::renet::RenetClient;
use shared::messages::{ChatMessageRequest, ClientToServerMessage, SaveWorldRequest};
use shared::world::BlockData;

use super::SendGameMessageExtension;

pub enum NetworkAction {
    ChatMessage(String),
    WorldUpdateRequest {
        requested_chunks: Vec<IVec3>,
        player_chunk_pos: IVec3,
        render_distance: u32,
    },
    SaveWorldRequest,
    BlockInteraction {
        position: IVec3,
        block_type: Option<BlockData>, // None = delete, Some = add
    },
}

pub fn send_network_action(client: &mut ResMut<RenetClient>, action: NetworkAction) {
    match action {
        NetworkAction::ChatMessage(msg) => {
            let input_message =
                ClientToServerMessage::ChatMessage(ChatMessageRequest { content: msg });

            client.send_game_message(input_message);
        }
        NetworkAction::WorldUpdateRequest {
            requested_chunks,
            player_chunk_pos,
            render_distance,
        } => {
            let input_message = ClientToServerMessage::WorldUpdateRequest {
                player_chunk_position: player_chunk_pos,
                requested_chunks,
                render_distance,
            };

            client.send_game_message(input_message);
        }
        NetworkAction::SaveWorldRequest => {
            let save_request =
                ClientToServerMessage::SaveWorldRequest(SaveWorldRequest { session_token: 0 });

            client.send_game_message(save_request);
        }
        NetworkAction::BlockInteraction {
            position,
            block_type,
        } => {
            let message = ClientToServerMessage::BlockInteraction {
                position,
                block_type,
            };

            client.send_game_message(message);
        }
    }
}
