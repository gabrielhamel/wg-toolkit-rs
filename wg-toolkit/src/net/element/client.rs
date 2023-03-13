//! Definition of the elements that can be sent from server to client
//! once connected to the base application..


use std::io::{self, Write, Read};

use glam::Vec3A;

use crate::util::io::*;

use super::{SimpleElement, TopElement, EmptyElement, ElementLength};


/// The server informs us how frequently it is going to send to us.
#[derive(Debug)]
pub struct UpdateFrequencyNotification {
    /// The frequency in hertz.
    pub frequency: u8,
}

impl UpdateFrequencyNotification {
    pub const ID: u8 = 0x02;
}

impl SimpleElement for UpdateFrequencyNotification {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.frequency)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { frequency: read.read_u8()? })
    }
}

impl TopElement for UpdateFrequencyNotification {
    const LEN: ElementLength = ElementLength::Fixed(7);
}


/// The server informs us of the current (server) game time.
#[derive(Debug)]
pub struct SetGameTime {
    /// The server game time.
    pub game_time: u32,
}

impl SetGameTime {
    pub const ID: u8 = 0x03;
}

impl SimpleElement for SetGameTime {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.game_time)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { game_time: read.read_u32()? })
    }

}

impl TopElement for SetGameTime {
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// The server wants to resets the entities in the Area of Interest (AoI).
#[derive(Debug)]
pub struct ResetEntities {
    pub keep_player_on_base: bool,
}

impl ResetEntities {
    pub const ID: u8 = 0x04;
}

impl SimpleElement for ResetEntities {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.keep_player_on_base as _)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { keep_player_on_base: read.read_u8()? != 0 })
    }

}

impl TopElement for ResetEntities {
    const LEN: ElementLength = ElementLength::Fixed(1);
}


/// Sent from the base when a player should be created, the entity id
/// is given with its type.
/// 
/// The remaining data will later be decoded properly depending on the
/// entity type, it's used for initializing its properties (TODO).
/// For example the `Login` entity receive the account UID.
#[derive(Debug)]
pub struct CreateBasePlayer {
    pub entity_id: u32,
    pub entity_type: u16,
    pub entity_data: Vec<u8>,
}

impl CreateBasePlayer {
    pub const ID: u8 = 0x05;
}

impl SimpleElement for CreateBasePlayer {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.entity_id)?;
        write.write_u16(self.entity_type)?;
        write.write_blob(&self.entity_data)
    }

    fn decode<R: Read>(mut read: R, len: usize) -> io::Result<Self> {
        Ok(Self {
            entity_id: read.read_u32()?,
            entity_type: read.read_u16()?,
            entity_data: read.read_blob(len - 6)?,
        })
    }
}

impl TopElement for CreateBasePlayer {
    const LEN: ElementLength = ElementLength::Variable16;
}


// TODO: 0x06: CreateCellPlayer
// TODO: 0x07: DummyPacket
// TODO: 0x08: SpaceProperty
// TODO: 0x09: AddSpaceGeometryMapping
// TODO: 0x0A: RemoveSpaceGeometryMapping
// TODO: 0x0B: CreateEntity
// TODO: 0x0C: CreateEntityDetailed


/// It is used as a timestamp for the elements in a bundle.
#[derive(Debug)]
pub struct TickSync {
    pub tick: u8,
}

impl TickSync {
    pub const ID: u8 = 0x13;
}

impl SimpleElement for TickSync {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.tick)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { tick: read.read_u8()? })
    }

}

impl TopElement for TickSync {
    const LEN: ElementLength = ElementLength::Fixed(1);
}


/// Sent by the server to inform that subsequent elements will target
/// the player entity.
#[derive(Debug, Default)]
pub struct SelectPlayerEntity;

impl SelectPlayerEntity {
    pub const ID: u8 = 0x1A;
}

impl EmptyElement for SelectPlayerEntity {}


/// This is when an update is being forced back for an (ordinarily)
/// client controlled entity, including for the player. Usually this is
/// due to a physics correction from the server, but it could be for any
/// reason decided by the server (e.g. server-initiated teleport).
#[derive(Debug)]
pub struct ForcedPosition {
    pub entity_id: u32,
    pub space_id: u32,
    pub vehicle_entity_id: u32,
    pub position: Vec3A,
    pub direction: Vec3A,
}

impl ForcedPosition {
    pub const ID: u8 = 0x1B;
}

impl SimpleElement for ForcedPosition {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.entity_id)?;
        write.write_u32(self.space_id)?;
        write.write_u32(self.vehicle_entity_id)?;
        write.write_vec3(self.position)?;
        write.write_vec3(self.direction)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            entity_id: read.read_u32()?,
            space_id: read.read_u32()?,
            vehicle_entity_id: read.read_u32()?,
            position: read.read_vec3()?,
            direction: read.read_vec3()?,
        })
    }

}


/// A call to a selected entity's method.
#[derive(Debug)]
pub struct EntityMethod {
    
}

impl EntityMethod {
    pub const FIRST_ID: u8 = 0x4E;
    pub const LAST_ID: u8 = 0xA6;
}


/// Setting a selected entity's property value.
#[derive(Debug)]
pub struct EntityProperty {

}

impl EntityProperty {
    pub const FIRST_ID: u8 = 0xA7;
    pub const LAST_ID: u8 = 0xFE;
}


/// An avatar update.
#[derive(Debug)]
pub struct AvatarUpdate {
    pub id: AvatarUpdateId,
    /// Position X, Y, Z.
    pub pos: Vec3A,
    /// Direction Yaw, Pitch, Roll.
    pub dir: Vec3A,
}

/// The entity ID for an avatar update.
#[derive(Debug)]
pub enum AvatarUpdateId {
    /// The entity ID is given directly without aliasing.
    NoAlias(u32),
    /// An alias for the entity ID, referring to an internal table of alias.
    Alias(u8),
}
