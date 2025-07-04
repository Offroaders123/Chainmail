use injectables::inject_fields;

use crate::java::indev::item::SlottedItem;
use crate::nbt::tag::{IntTag, ListTag, StringTag};

pub enum BlockEntity {
    Chest(Chest),
    Furnace(Furnace),
}

#[allow(non_snake_case)]
pub struct BlockEntityLike {
    Pos: IntTag, // not a tuple, interestingly enough, just a composed int of some sort, I think you have to use math to get the actual coordinates out of the value
    id: StringTag, // should this be a generic of `BlockEntityResource`?
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct Chest {
    Items: ListTag<SlottedItem>,
}

#[allow(non_snake_case)]
#[inject_fields(BlockEntityLike)]
pub struct Furnace {
    BurnTime: ShortTag,
    CookTime: ShortTag,
    Items: [Option<SlottedItem>; 3],
}

pub enum BlockEntityResource {
    Chest,
    Furnace,
}
