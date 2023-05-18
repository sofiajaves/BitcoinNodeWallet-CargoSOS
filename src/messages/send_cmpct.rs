use super::{
    message::Message,
    command_name::CommandName,
};

use crate::serialization::{
    serializable_little_endian::SerializableLittleEndian,
    serializable_internal_order::SerializableInternalOrder,
    deserializable_little_endian::DeserializableLittleEndian,
    deserializable_internal_order::DeserializableInternalOrder,
    error_serialization::ErrorSerialization,
};

use std::io::{
    Read, 
    Write
};

#[derive(Debug, std::cmp::PartialEq)]
pub struct SendCmpctMessage {
    pub announce: bool,
    pub version: u64,
}

impl Message for SendCmpctMessage {

    fn get_command_name() -> CommandName {
        CommandName::SendCmpct
    }
}

impl SerializableInternalOrder for SendCmpctMessage {

    fn io_serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.announce.le_serialize(stream)?;
        self.version.le_serialize(stream)?;
        Ok(())
    }
}

impl DeserializableInternalOrder for SendCmpctMessage {
    
    fn io_deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {        
        Ok(SendCmpctMessage{
            announce: bool::le_deserialize(stream)?,
            version: u64::le_deserialize(stream)?,
        })
    }
}