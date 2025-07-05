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
pub struct EntityLike {
    pub Air: ShortTag,
    pub FallDistance: FloatTag,
    pub Fire: ShortTag,
    pub Motion: [FloatTag; 3],
    pub Pos: [FloatTag; 3],
    pub Rotation: [FloatTag; 2],
    pub id: StringTag, // should this be generic for `EntityResource`?
}

#[allow(non_snake_case)]
pub struct MobLike {
    pub AttackTime: ShortTag,
    pub DeathTime: ShortTag,
    pub Health: ShortTag,
    pub HurtTime: ShortTag,
}

#[allow(non_snake_case)]
// #[inject_fields(EntityLike)]
pub struct Arrow {
    pub inGround: BooleanTag,
    pub inTile: BooleanTag,
    pub shake: BooleanTag,
    pub xTile: ShortTag,
    pub yTile: ShortTag,
    pub zTile: ShortTag,
}

// #[inject_fields(EntityLike, MobLike)]
pub struct Creeper {}

// #[inject_fields(EntityLike, MobLike)]
pub struct Giant {}

#[allow(non_snake_case)]
// #[inject_fields(EntityLike)]
pub struct ItemEntity {
    pub Age: ShortTag,
    pub Item: Item,
}

#[allow(non_snake_case)]
// #[inject_fields(EntityLike, MobLike)]
pub struct LocalPlayer {
    pub Score: IntTag,
    pub Inventory: ListTag<SlottedItem>,
}

// yeah weird that it's moblike, I know
#[allow(non_snake_case)]
// #[inject_fields(EntityLike, Omit<MobLike, "Health">)]
pub struct Painting {
    pub Dir: ByteTag<PaintingDirection>,
    pub Motive: PaintingVariant,
    pub TileY: IntTag,
    pub TileZ: IntTag,
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

// #[inject_fields(EntityLike, MobLike)]
pub struct Pig {}

// #[inject_fields(EntityLike, MobLike)]
pub struct Skeleton {}

// #[inject_fields(EntityLike, MobLike)]
pub struct Spider {}

#[allow(non_snake_case)]
// #[inject_fields(EntityLike, Omit<MobLike, "Health">)]
pub struct PrimedTnt {
    pub Fuse: ByteTag,
}

// #[inject_fields(EntityLike, MobLike)]
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
