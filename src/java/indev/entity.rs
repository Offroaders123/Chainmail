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
pub struct Arrow {
    pub entity_like: EntityLike,
    pub inGround: BooleanTag,
    pub inTile: BooleanTag,
    pub shake: BooleanTag,
    pub xTile: ShortTag,
    pub yTile: ShortTag,
    pub zTile: ShortTag,
}

pub struct Creeper {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Giant {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
pub struct ItemEntity {
    pub entity_like: EntityLike,
    pub Age: ShortTag,
    pub Item: Item,
}

#[allow(non_snake_case)]
pub struct LocalPlayer {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
    pub Score: IntTag,
    pub Inventory: ListTag<SlottedItem>,
}

// yeah weird that it's moblike, I know
#[allow(non_snake_case)]
pub struct Painting {
    // <--------- this needs to be adjusted!!! paintings don't have the Health property, but they are MobLike
    pub entity_like: EntityLike, // Omit<MobLike, "Health">
    pub Dir: ByteTag<PaintingDirection>,
    pub Motive: StringTag<PaintingVariant>,
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

pub struct Pig {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Skeleton {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Spider {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
pub struct PrimedTnt {
    // <--------- this needs to be adjusted!!! paintings don't have the Health property, but they are MobLike
    pub entity_like: EntityLike, // Omit<MobLike, "Health">
    pub Fuse: ByteTag,
}

pub struct Zombie {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

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
