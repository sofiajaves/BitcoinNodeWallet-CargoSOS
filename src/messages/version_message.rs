use super::{
    serializable::Serializable,
    deserializable::Deserializable,
    error_message::ErrorMessage,
};

use std::net::Ipv6Addr;
use chrono::{
    DateTime,
    Timelike,
    NaiveDateTime,
    offset::Utc
};

use std::io::{Read, Write};

use crate::connections::{
    p2p_protocol::ProtocolVersionP2P,
    suppored_services::SupportedServices,
};

pub const VERSION_TYPE: [u8; 12] = [118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 1];

pub struct VersionMessage {
    pub version: ProtocolVersionP2P,
    pub services: SupportedServices,
    pub timestamp: DateTime<Utc>,
    pub recv_services: SupportedServices,
    pub recv_addr: Ipv6Addr,
    pub recv_port: u16,
    pub trans_addr: Ipv6Addr,
    pub trans_port: u16, // tal vez es el mismo que el recv_port
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}

impl VersionMessage {

    pub fn new(
        version: ProtocolVersionP2P,
        services: SupportedServices,
        timestamp: DateTime<Utc>,
        recv_services: SupportedServices,
        recv_addr: Ipv6Addr,
        recv_port: u16,
        trans_addr: Ipv6Addr,
        trans_port: u16,
        nonce: u64,
        user_agent: String,
        start_height: i32,
        relay: bool,
    ) -> Self {
        todo!();
    }
}

impl Serializable for VersionMessage {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage>{

    
        //version
        if stream.write(&self.version.to_i32().to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorWhileWriting);
        }
            
        //services
        /*let services: &[u64] = match self.services.try_into(){
            Ok(services) => services,
            _ => return Err(ErrorMessage::ErrorWhileWriting),
        };

        if stream.write(services).is_err() {
            Ok(services) => services,
            _ => return Err(ErrorMessage::ErrorInSerialization),
            
        }*/

        //timestamp
        let timestamp_bytes = self.timestamp.timestamp().to_le_bytes();
        if stream.write(&timestamp_bytes).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //recv_services
        /*if stream.write(&self.recv_services.to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }*/

        //recv_addr
        let recv_bytes = self.recv_addr.octets();
        if stream.write(&recv_bytes).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //recv_port
        if stream.write(&self.recv_port.to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }
        
        //trans addr
        let trans_addr_bytes = self.trans_addr.octets();
        if stream.write(&trans_addr_bytes).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //trans port
        if stream.write(&self.trans_port.to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //nonce
        if stream.write(&self.nonce.to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //user_agent VER ESTO
        if stream.write(&self.user_agent.as_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //start_height
        if stream.write(&self.start_height.to_le_bytes()).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        //relay
        let relay_value = match self.relay {
            true => 0x01,
            false => 0x00,
        };
        if stream.write(&[relay_value]).is_err() {
            return Err(ErrorMessage::ErrorInSerialization);
        }

        Ok(())

        }


}

impl Deserializable for VersionMessage {
    type Value = Self;
    fn deserialize(stream: &mut dyn Read) ->  Result<Self::Value, ErrorMessage> {
        //version
        let mut version_bytes = [0u8; 4];
        if stream.read_exact(&mut version_bytes).is_err() {
            return Err(ErrorMessage::ErrorInDeserialization);
        }
        let version_int = i32::from_le_bytes(version_bytes);
        if ProtocolVersionP2P::from_i32(version_int).is_err(){
            return Err(ErrorMessage::ErrorInDeserialization);
        };
        
        //services
        //para despues

        //timestamp
        /*let mut timestamp_bytes = [0u8; 8];
        if stream.read_exact(&mut timestamp_bytes).is_err() {
        return Err(ErrorMessage::ErrorInDeserialization);
        }
        let timestamp_int = i64::from_le_bytes(timestamp_bytes);
        //let timestamp_utc = Utc.timestamp(timestamp_int, 0);
        let timestamp_utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_int, 0), Utc);
        let timestamp = DateTime::<Utc>::from_utc(timestamp_utc, Utc);*/
        
        let mut timestamp_bytes = [0u8; 8];
        if stream.read_exact(&mut timestamp_bytes).is_err() {
            return Err(ErrorMessage::ErrorInDeserialization);
        }
        let timestamp_int = i64::from_le_bytes(timestamp_bytes);
        let timestamp_utc = NaiveDateTime::from_timestamp_opt(timestamp_int, 0).ok_or(ErrorMessage::ErrorInDeserialization)?;
        let timestamp = DateTime::<Utc>::from_utc(timestamp_utc, Utc);








        todo!()
    }
}