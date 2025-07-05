use crate::java::alpha::{entity::EntityResource, item::SlottedItem};
use crate::nbt::tag::{IntTag, ShortTag, StringTag};

pub enum BlockEntity {
    Chest(Chest),
    Furnace(Furnace),
    MobSpawner(MobSpawner),
    Sign(Sign),
}

#[allow(non_snake_case)]
pub struct BlockEntityLike {
    pub id: StringTag, // should this be a generic to `BlockEntityResource`?
    pub x: IntTag,
    pub y: IntTag,
    pub z: IntTag,
}

#[allow(non_snake_case)]
pub struct Chest {
    pub block_entity_like: BlockEntityLike,
    pub Items: SlottedItem,
}

#[allow(non_snake_case)]
pub struct Furnace {
    pub block_entity_like: BlockEntityLike,
    pub BurnTime: ShortTag,
    pub CookTime: ShortTag,
    pub Items: [Option<SlottedItem>; 3],
}

#[allow(non_snake_case)]
pub struct MobSpawner {
    pub block_entity_like: BlockEntityLike,
    pub EntityId: StringTag<EntityResource>,
    pub Delay: ShortTag,
}

#[allow(non_snake_case)]
pub struct Sign {
    pub block_entity_like: BlockEntityLike,
    pub Text1: StringTag,
    pub Text2: StringTag,
    pub Text3: StringTag,
    pub Text4: StringTag,
}

pub enum BlockEntityResource {
    Chest,
    Furnace,
    MobSpawner,
    Sign,
}
