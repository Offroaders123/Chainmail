use crate::java::alpha::{block_entity::BlockEntity, entity::Entity};
use crate::nbt::tag::{BooleanTag, ByteArrayTag, IntTag, ListTag, LongTag};

#[allow(non_snake_case)]
pub struct Chunk {
    xPos: IntTag,
    zPos: IntTag,
    TerrainPopulated: BooleanTag,
    LastUpdate: LongTag,
    Blocks: ByteArrayTag,
    Data: ByteArrayTag,
    BlockLight: ByteArrayTag,
    SkyLight: ByteArrayTag,
    HeightMap: ByteArrayTag,
    Entities: ListTag<Entity>,
    TileEntities: ListTag<BlockEntity>,
}
