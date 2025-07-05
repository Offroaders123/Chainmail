use crate::nbt::tag::{IntTag, ListTag, LongTag, StringTag};
use crate::pi::v0_1_1::{
    entity::{Entity, Player},
    tile_entity::TileEntity,
};

#[allow(non_snake_case)]
pub struct LevelDat {
    pub GameType: IntTag<GameMode>,
    pub LastPlayed: LongTag,
    pub LevelName: StringTag,
    pub Platform: IntTag, // might be an enum of values; likely the same case for Bedrock types too, since this is PE essentially
    pub Player: Player,
    pub RandomSeed: LongTag,
    pub SizeOnDisk: LongTag,
    pub SpawnX: IntTag,
    pub SpawnY: IntTag,
    pub SpawnZ: IntTag,
    pub StorageVersion: IntTag<StorageVersion>, // always 3?
    pub Time: LongTag,
    pub dayCycleStopTime: LongTag,
    pub spawnMobs: IntTag<SpawnMobs>,
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
    pub Entities: ListTag<Entity>,
    pub TileEntities: ListTag<TileEntity>,
}
