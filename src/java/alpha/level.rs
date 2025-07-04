use crate::java::alpha::entity::Player;
use crate::nbt::tag::{IntTag, LongTag};

#[allow(non_snake_case)]
pub struct Level {
    Data: Data,
}

#[allow(non_snake_case)]
pub struct Data {
    LastPlayed: LongTag,
    SizeOnDisk: LongTag,
    RandomSeed: LongTag,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    Time: LongTag,
    Player: Player,
}
