use serde::{Deserialize, Serialize};

use crate::java::indev::{block_entity::BlockEntity, entity::Entity};
use crate::nbt::tag::{ByteArrayTag, ByteTag, IntTag, ListTag, LongTag, ShortTag, StringTag};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Level {
    pub Environment: Environment,
    pub Map: Map,
    pub TileEntities: ListTag<BlockEntity>,
    pub About: About,
    pub Entities: ListTag<Entity>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub SurroundingGroundHeight: ShortTag,
    pub TimeOfDay: ShortTag,
    pub CloudHeight: ShortTag,
    pub CloudColor: IntTag,
    pub SkyBrightness: ByteTag,
    pub SkyColor: IntTag,
    pub FogColor: IntTag,
    pub SurroundingWaterHeight: ShortTag,
    pub SurroundingGroundType: ByteTag,
    pub SurroundingWaterType: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Map {
    pub Blocks: ByteArrayTag,
    pub Length: ShortTag,
    pub Height: ShortTag,
    pub Data: ByteArrayTag,
    pub Width: ShortTag,
    pub Spawn: [ShortTag; 3],
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct About {
    pub Author: StringTag,
    pub CreatedOn: LongTag,
    pub Name: StringTag,
}
