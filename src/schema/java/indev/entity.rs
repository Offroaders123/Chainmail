use serde::{Deserialize, Serialize};

use crate::{
    nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, ShortTag, StringTag},
    schema::java::indev::item::{Item, SlottedItem},
};

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct MobLike {
    pub AttackTime: ShortTag,
    pub DeathTime: ShortTag,
    pub Health: ShortTag,
    pub HurtTime: ShortTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Arrow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub inGround: BooleanTag,
    pub inTile: BooleanTag,
    pub shake: BooleanTag,
    pub xTile: ShortTag,
    pub yTile: ShortTag,
    pub zTile: ShortTag,
}

#[derive(Serialize, Deserialize)]
pub struct Creeper {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Giant {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ItemEntity {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Age: ShortTag,
    pub Item: Item,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LocalPlayer {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Score: IntTag,
    pub Inventory: ListTag<SlottedItem>,
}

// yeah weird that it's moblike, I know
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Painting {
    // <--------- this needs to be adjusted!!! paintings don't have the Health property, but they are MobLike
    #[serde(flatten)]
    pub entity_like: EntityLike, // Omit<MobLike, "Health">
    pub Dir: ByteTag<PaintingDirection>,
    pub Motive: StringTag<PaintingVariant>,
    pub TileY: IntTag,
    pub TileZ: IntTag,
}

#[derive(Serialize, Deserialize)]
pub enum PaintingDirection {
    East = 0,
    North,
    West,
    South,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Pig {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Skeleton {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Spider {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PrimedTnt {
    // <--------- this needs to be adjusted!!! paintings don't have the Health property, but they are MobLike
    #[serde(flatten)]
    pub entity_like: EntityLike, // Omit<MobLike, "Health">
    pub Fuse: ByteTag,
}

#[derive(Serialize, Deserialize)]
pub struct Zombie {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
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
