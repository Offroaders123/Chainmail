use serde::{Deserialize, Serialize};

use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, ShortTag};
use crate::pi::v0_1_1::{dimension::DimensionID, item::Item};

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct EntityLike /*<EntityID extends number | undefined>*/ {
    pub id: IntTag<EntityResource>, // id: EntityID extends number ? IntTag<EntityID> : EntityID;
    pub Pos: [FloatTag; 3],
    pub Motion: [FloatTag; 3], // doesn't seem to mention optional
    pub Rotation: [FloatTag; 2],
    pub FallDistance: FloatTag,
    pub Fire: ShortTag,
    pub Air: ShortTag,
    pub OnGround: BooleanTag,
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
pub struct AnimalLike {
    pub Age: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Particle {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Player /*extends EntityLike<undefined>, MobLike*/ {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Armor: ListTag<Item>, // no slot tag actually, ignore. // with slot tag, right? docs doesn't say here. and are these optional? it's actually `[Item, Item, Item, Item]`
    pub Dimension: IntTag<DimensionID>,
    pub Inventory: ListTag<Item>, // slot tag for sure
    pub Score: IntTag,
    pub Sleeping: BooleanTag,
    pub SleepTimer: ShortTag,
    pub SpawnX: IntTag,
    pub SpawnY: IntTag,
    pub SpawnZ: IntTag,
    pub abilities: PlayerAbilities,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PlayerAbilities {
    pub mayfly: BooleanTag,
    pub flying: BooleanTag,
    pub invulnerable: BooleanTag,
    pub mayBuild: BooleanTag,
    pub instabuild: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct TripodCamera {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct CameraEntity {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Chicken {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub animal_like: AnimalLike,
}

#[derive(Serialize, Deserialize)]
pub struct Cow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub animal_like: AnimalLike,
}

#[derive(Serialize, Deserialize)]
pub struct Pig {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub animal_like: AnimalLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sheep {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub animal_like: AnimalLike,
    pub Sheared: BooleanTag,
    pub Color: ByteTag<SheepWoolColor>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Zombie {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Creeper {
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

#[derive(Serialize, Deserialize)]
pub struct PigZombie {
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
    pub Health: ShortTag,
    pub Age: ShortTag,
    pub Item: Item,
}

#[derive(Serialize, Deserialize)]
pub struct PrimedTNT {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct FallingTile {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Arrow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Snowball {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct ThrownEgg {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Painting {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
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
