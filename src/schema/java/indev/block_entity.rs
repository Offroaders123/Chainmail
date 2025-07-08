use serde::{Deserialize, Serialize};

use crate::schema::java::indev::item::SlottedItem;
use crate::nbt::tag::{IntTag, ListTag, ShortTag, StringTag};

#[derive(Serialize, Deserialize)]
pub enum BlockEntity {
    Chest(Chest),
    Furnace(Furnace),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlockEntityLike {
    pub Pos: IntTag, // not a tuple, interestingly enough, just a composed int of some sort, I think you have to use math to get the actual coordinates out of the value
    pub id: StringTag, // should this be a generic of `BlockEntityResource`?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chest {
    #[serde(flatten)]
    pub block_entity_like: BlockEntityLike,
    pub Items: ListTag<SlottedItem>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Furnace {
    #[serde(flatten)]
    pub block_entity_like: BlockEntityLike,
    pub BurnTime: ShortTag,
    pub CookTime: ShortTag,
    pub Items: [Option<SlottedItem>; 3],
}

pub enum BlockEntityResource {
    Chest,
    Furnace,
}
