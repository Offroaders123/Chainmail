use injectables::{inject_fields, injectable};

use crate::java::indev::item::{Item, SlottedItem};
use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, ShortTag, StringTag};

pub enum Entity {
    Arrow(Arrow),
    Creeper(Creeper),
    Giant(Giant),
    Item(ItemEntity),
    LocalPlayer(LocalPlayer),
    Painting(Painting),
    Pig(Pig),
    PrimedTnt(PrimedTnt),
    Skeleton(Skeleton),
    Spider(Spider),
    Zombie(Zombie),
}

#[allow(non_snake_case)]
#[injectable]
pub struct EntityLike {
    Air: ShortTag,
    FallDistance: FloatTag,
    Fire: ShortTag,
    Motion: [FloatTag; 3],
    Pos: [FloatTag; 3],
    Rotation: [FloatTag; 2],
    id: StringTag, // should this be generic for `EntityResource`?
}

#[allow(non_snake_case)]
#[injectable]
pub struct MobLike {
    AttackTime: ShortTag,
    DeathTime: ShortTag,
    Health: ShortTag,
    HurtTime: ShortTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct Arrow {
    inGround: BooleanTag,
    inTile: BooleanTag,
    shake: BooleanTag,
    xTile: ShortTag,
    yTile: ShortTag,
    zTile: ShortTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Creeper {}

#[inject_fields(EntityLike, MobLike)]
pub struct Giant {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct ItemEntity {
    Age: ShortTag,
    Item: Item,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct LocalPlayer {
    Score: IntTag,
    Inventory: ListTag<SlottedItem>,
}

// yeah weird that it's moblike, I know
#[allow(non_snake_case)]
#[inject_fields(EntityLike, Omit<MobLike, "Health">)]
pub struct Painting {
    Dir: ByteTag<PaintingDirection>,
    Motive: PaintingVariant,
    TileY: IntTag,
    TileZ: IntTag,
}

pub enum PaintingDirection {
    East = 0,
    North,
    West,
    South,
}

pub enum PaintingVariant {
    Alban,
    Aztec,
    Aztec2,
    Bomb,
    Bust,
    Courbet,
    Kebab,
    Match,
    Plant,
    Pool,
    Sea,
    SkullAndRoses,
    Stage,
    Void,
    Wanderer,
    Wasteland,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Pig {}

#[inject_fields(EntityLike, MobLike)]
pub struct Skeleton {}

#[inject_fields(EntityLike, MobLike)]
pub struct Spider {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, Omit<MobLike, "Health">)]
pub struct PrimedTnt {
    Fuse: ByteTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Zombie {}

pub enum EntityResource {
    Arrow,
    Creeper,
    Giant,
    Item,
    LocalPlayer,
    Painting,
    Pig,
    PrimedTnt,
    Skeleton,
    Spider,
    Zombie,
}
