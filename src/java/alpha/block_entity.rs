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
// #[inject_fields(BlockEntityLike)]
pub struct Chest {
    pub Items: SlottedItem,
}

#[allow(non_snake_case)]
// #[inject_fields(BlockEntityLike)]
pub struct Furnace {
    pub BurnTime: ShortTag,
    pub CookTime: ShortTag,
    pub Items: [Option<SlottedItem>; 3],
}

#[allow(non_snake_case)]
// #[inject_fields(BlockEntityLike)]
pub struct MobSpawner {
    pub EntityId: StringTag<EntityResource>,
    pub Delay: ShortTag,
}

#[allow(non_snake_case)]
// #[inject_fields(BlockEntityLike)]
pub struct Sign {
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
