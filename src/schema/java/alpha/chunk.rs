use serde::{Deserialize, Serialize};

use crate::{
    nbt::tag::{BooleanTag, ByteArrayTag, IntTag, ListTag, LongTag},
    schema::java::alpha::{block_entity::BlockEntity, entity::Entity},
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub xPos: IntTag,
    pub zPos: IntTag,
    pub TerrainPopulated: BooleanTag,
    pub LastUpdate: LongTag,
    pub Blocks: ByteArrayTag,
    pub Data: ByteArrayTag,
    pub BlockLight: ByteArrayTag,
    pub SkyLight: ByteArrayTag,
    pub HeightMap: ByteArrayTag,
    pub Entities: ListTag<Entity>,
    pub TileEntities: ListTag<BlockEntity>,
}
