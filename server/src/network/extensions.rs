use bevy::prelude::*;
use bevy_renet::renet::{ClientId, DefaultChannel, RenetServer};
use shared::utils::format_bytes;

pub trait SendGameMessageExtension {
    fn send_game_message(
        &mut self,
        client_id: ClientId,
        message: shared::messages::ServerToClientMessage,
    );
    fn broadcast_game_message(&mut self, message: shared::messages::ServerToClientMessage);
    fn receive_game_message(
        &mut self,
        client_id: ClientId,
    ) -> Result<shared::messages::ClientToServerMessage, Box<bincode::ErrorKind>>;
}

impl SendGameMessageExtension for RenetServer {
    fn send_game_message(
        &mut self,
        client_id: ClientId,
        message: shared::messages::ServerToClientMessage,
    ) {
        let payload = shared::game_message_to_payload(message);
        let size = payload.len() as u64;
        if size > (10 * 1024) {
            info!("Sending game message of size: {}", format_bytes(size));
        }
        self.send_message(client_id, DefaultChannel::ReliableOrdered, payload);
    }

    fn broadcast_game_message(&mut self, message: shared::messages::ServerToClientMessage) {
        let payload = shared::game_message_to_payload(message);
        let size = payload.len() as u64;
        if size > (10 * 1024) {
            info!("Broadcasting game message of size: {}", format_bytes(size));
        }
        self.broadcast_message(DefaultChannel::ReliableOrdered, payload);
    }

    fn receive_game_message(
        &mut self,
        client_id: ClientId,
    ) -> Result<shared::messages::ClientToServerMessage, Box<bincode::ErrorKind>> {
        let payload = self.receive_message(client_id, DefaultChannel::ReliableOrdered);
        if let Some(payload) = payload {
            // debug!("Received payload: {:?}", payload);
            let msg = shared::payload_to_game_message::<shared::messages::ClientToServerMessage>(
                &payload,
            );
            match msg {
                Ok(msg) => {
                    // info!("Received message: {:?}", msg);
                    return Ok(msg);
                }
                Err(e) => {
                    warn!("Error deserializing message: {:?}", e);
                    return Err(e);
                }
            }
        }
        Err(Box::new(bincode::ErrorKind::Custom(
            "No message received".to_string(),
        )))
    }
}
