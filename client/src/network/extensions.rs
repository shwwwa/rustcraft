use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;

pub trait SendGameMessageExtension {
    fn send_game_message(&mut self, message: shared::messages::ClientToServerMessage);
}

impl SendGameMessageExtension for RenetClient {
    fn send_game_message(&mut self, message: shared::messages::ClientToServerMessage) {
        let payload = bincode::options().serialize(&message).unwrap();
        self.send_message(DefaultChannel::ReliableOrdered, payload);
    }
}
