use injectables::{inject_fields, injectable};

use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, ShortTag};
use crate::pi::v0_1_1::{dimension::DimensionID, item::Item};

pub enum Entity {
    Particle(Particle),
    Player(Player),
    TripodCamera(TripodCamera),
    CameraEntity(CameraEntity),
    Chicken(Chicken),
    Cow(Cow),
    Pig(Pig),
    Sheep(Sheep),
    Zombie(Zombie),
    Creeper(Creeper),
    Skeleton(Skeleton),
    Spider(Spider),
    PigZombie(PigZombie),
    ItemEntity(ItemEntity),
    PrimedTNT(PrimedTNT),
    FallingTile(FallingTile),
    Arrow(Arrow),
    Snowball(Snowball),
    ThrownEgg(ThrownEgg),
    Painting(Painting),
}

#[allow(non_snake_case)]
#[injectable]
pub struct EntityLike /*<EntityID extends number | undefined>*/ {
    id: IntTag<EntityResource>, // id: EntityID extends number ? IntTag<EntityID> : EntityID;
    Pos: [FloatTag; 3],
    Motion: [FloatTag; 3], // doesn't seem to mention optional
    Rotation: [FloatTag; 2],
    FallDistance: FloatTag,
    Fire: ShortTag,
    Air: ShortTag,
    OnGround: BooleanTag,
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
#[injectable]
pub struct AnimalLike {
    Age: IntTag,
}

#[inject_fields(EntityLike)]
pub struct Particle {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Player /*extends EntityLike<undefined>, MobLike*/ {
    Armor: ListTag<Item>, // no slot tag actually, ignore. // with slot tag, right? docs doesn't say here. and are these optional? it's actually `[Item, Item, Item, Item]`
    Dimension: IntTag<DimensionID>,
    Inventory: ListTag<Item>, // slot tag for sure
    Score: IntTag,
    Sleeping: BooleanTag,
    SleepTimer: ShortTag,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    abilities: PlayerAbilities,
}

#[allow(non_snake_case)]
pub struct PlayerAbilities {
    mayfly: BooleanTag,
    flying: BooleanTag,
    invulnerable: BooleanTag,
    mayBuild: BooleanTag,
    instabuild: BooleanTag,
}

#[inject_fields(EntityLike)]
pub struct TripodCamera {}

#[inject_fields(EntityLike)]
pub struct CameraEntity {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Chicken {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Cow {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Pig {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Sheep {
    Sheared: BooleanTag,
    Color: ByteTag<SheepWoolColor>,
}

pub enum SheepWoolColor {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Zombie {}

#[inject_fields(EntityLike, MobLike)]
pub struct Creeper {}

#[inject_fields(EntityLike, MobLike)]
pub struct Skeleton {}

#[inject_fields(EntityLike, MobLike)]
pub struct Spider {}

#[inject_fields(EntityLike, MobLike)]
pub struct PigZombie {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct ItemEntity {
    Health: ShortTag,
    Age: ShortTag,
    Item: Item,
}

#[inject_fields(EntityLike)]
pub struct PrimedTNT {}

#[inject_fields(EntityLike)]
pub struct FallingTile {}

#[inject_fields(EntityLike)]
pub struct Arrow {}

#[inject_fields(EntityLike)]
pub struct Snowball {}

#[inject_fields(EntityLike)]
pub struct ThrownEgg {}

#[inject_fields(EntityLike)]
pub struct Painting {}

pub enum EntityResource {
    // these first four are all actually `0`, this needs to be fixed with a `Into<u8>` block, or something similar.
    Particle = 0,
    Player = 01,
    TripodCamera = 02,
    CameraEntity = 03,
    // <Animals>
    Chicken = 10,
    Cow,
    Pig,
    Sheep,
    // <Monsters>
    Zombie = 32,
    Creeper,
    Skeleton,
    Spider,
    PigZombie,
    // </Monsters>
    ItemEntity = 64,
    PrimedTnt,
    FallingTile,
    Arrow = 80,
    Snowball,
    ThrownEgg,
    Painting,
}
