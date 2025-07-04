use injectables::{inject_fields, injectable};

use crate::java::alpha::{entity::EntityResource, item::SlottedItem};
use crate::nbt::tag::{IntTag, ShortTag, StringTag};

pub enum BlockEntity {
    Chest(Chest),
    Furnace(Furnace),
    MobSpawner(MobSpawner),
    Sign(Sign),
}

#[allow(non_snake_case)]
#[injectable]
pub struct BlockEntityLike {
    id: StringTag, // should this be a generic to `BlockEntityResource`?
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct Chest {
    Items: SlottedItem,
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct Furnace {
    BurnTime: ShortTag,
    CookTime: ShortTag,
    Items: [Option<SlottedItem>; 3],
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct MobSpawner {
    EntityId: StringTag<EntityResource>,
    Delay: ShortTag,
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct Sign {
    Text1: StringTag,
    Text2: StringTag,
    Text3: StringTag,
    Text4: StringTag,
}

pub enum BlockEntityResource {
    Chest,
    Furnace,
    MobSpawner,
    Sign,
}
