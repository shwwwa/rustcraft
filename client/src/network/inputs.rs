use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use shared::messages::ClientToServerMessage;

use super::buffered_client::PlayerTickInputsBuffer;
use super::SendGameMessageExtension;

pub fn upload_player_inputs_system(
    mut client: ResMut<RenetClient>,
    mut inputs: ResMut<PlayerTickInputsBuffer>,
) {
    let mut frames = vec![];
    for input in inputs.buffer.iter() {
        frames.push(input.clone());
    }
    client.send_game_message(ClientToServerMessage::PlayerInputs(frames));
    inputs.buffer.clear();
}
