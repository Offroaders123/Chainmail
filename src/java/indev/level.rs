use crate::java::indev::{block_entity::BlockEntity, entity::Entity};
use crate::nbt::tag::{ByteArrayTag, ByteTag, IntTag, ListTag, LongTag, ShortTag, StringTag};

#[allow(non_snake_case)]
pub struct Level {
    Environment: Environment,
    Map: Map,
    TileEntities: ListTag<BlockEntity>,
    About: About,
    Entities: ListTag<Entity>,
}

#[allow(non_snake_case)]
pub struct Environment {
    SurroundingGroundHeight: ShortTag,
    TimeOfDay: ShortTag,
    CloudHeight: ShortTag,
    CloudColor: IntTag,
    SkyBrightness: ByteTag,
    SkyColor: IntTag,
    FogColor: IntTag,
    SurroundingWaterHeight: ShortTag,
    SurroundingGroundType: ByteTag,
    SurroundingWaterType: ByteTag,
}

#[allow(non_snake_case)]
pub struct Map {
    Blocks: ByteArrayTag,
    Length: ShortTag,
    Height: ShortTag,
    Data: ByteArrayTag,
    Width: ShortTag,
    Spawn: [ShortTag; 3],
}

#[allow(non_snake_case)]
pub struct About {
    Author: StringTag,
    CreatedOn: LongTag,
    Name: StringTag,
}
