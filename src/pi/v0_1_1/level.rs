use crate::nbt::tag::{IntTag, ListTag, LongTag, StringTag};
use crate::pi::v0_1_1::{
    entity::{Entity, Player},
    tile_entity::TileEntity,
};

#[allow(non_snake_case)]
pub struct LevelDat {
    GameType: IntTag<GameMode>,
    LastPlayed: LongTag,
    LevelName: StringTag,
    Platform: IntTag, // might be an enum of values; likely the same case for Bedrock types too, since this is PE essentially
    Player: Player,
    RandomSeed: LongTag,
    SizeOnDisk: LongTag,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    StorageVersion: IntTag<StorageVersion>, // always 3?
    Time: LongTag,
    dayCycleStopTime: LongTag,
    spawnMobs: IntTag<SpawnMobs>,
}

pub enum GameMode {
    Survival = 0,
    Creative,
}

pub enum StorageVersion {
    Version = 3,
}

pub enum SpawnMobs {
    True = 0,
    False,
}

#[allow(non_snake_case)]
pub struct EntityDat {
    Entities: ListTag<Entity>,
    TileEntities: ListTag<TileEntity>,
}
