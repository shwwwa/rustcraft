use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use shared::messages::ClientToServerMessage;

use crate::player::CurrentPlayerMarker;

use super::SendGameMessageExtension;

pub fn send_player_position_to_server(
    mut client: ResMut<RenetClient>,
    player: Query<&Transform, With<CurrentPlayerMarker>>,
) {
    let msg = ClientToServerMessage::SetPlayerPosition {
        position: player.single().translation,
    };
    client.send_game_message(msg);
}
