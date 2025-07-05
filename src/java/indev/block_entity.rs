use crate::java::indev::item::SlottedItem;
use crate::nbt::tag::{IntTag, ListTag, ShortTag, StringTag};

pub enum BlockEntity {
    Chest(Chest),
    Furnace(Furnace),
}

#[allow(non_snake_case)]
pub struct BlockEntityLike {
    pub Pos: IntTag, // not a tuple, interestingly enough, just a composed int of some sort, I think you have to use math to get the actual coordinates out of the value
    pub id: StringTag, // should this be a generic of `BlockEntityResource`?
}

#[allow(non_snake_case)]
// #[inject_fields(BlockEntityLike)]
pub struct Chest {
    pub Items: ListTag<SlottedItem>,
}

#[allow(non_snake_case)]
// #[inject_fields(BlockEntityLike)]
pub struct Furnace {
    pub BurnTime: ShortTag,
    pub CookTime: ShortTag,
    pub Items: [Option<SlottedItem>; 3],
}

pub enum BlockEntityResource {
    Chest,
    Furnace,
}
