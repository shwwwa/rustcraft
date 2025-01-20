use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::ErrorKind;
use shared::{
    game_message_to_payload,
    messages::{ClientToServerMessage, ServerToClientMessage},
};

pub trait SendGameMessageExtension {
    fn send_game_message(&mut self, message: ClientToServerMessage);
    fn receive_game_message(&mut self) -> Result<ServerToClientMessage, Box<ErrorKind>>;
}

impl SendGameMessageExtension for RenetClient {
    fn send_game_message(&mut self, message: ClientToServerMessage) {
        let payload = game_message_to_payload(message);
        self.send_message(DefaultChannel::ReliableOrdered, payload);
    }

    fn receive_game_message(&mut self) -> Result<ServerToClientMessage, Box<ErrorKind>> {
        let payload = self.receive_message(DefaultChannel::ReliableOrdered);
        if let Some(payload) = payload {
            // debug!("Received payload: {:?}", payload);
            let res = shared::payload_to_game_message::<ServerToClientMessage>(&payload);
            match res {
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
        Err(Box::new(ErrorKind::Custom(
            "No message received".to_string(),
        )))
    }
}
