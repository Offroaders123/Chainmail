use serde::{Deserialize, Serialize};

use crate::{
    nbt::tag::{IntTag, LongTag},
    schema::java::alpha::entity::Player,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Level {
    pub Data: Data,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub LastPlayed: LongTag,
    pub SizeOnDisk: LongTag,
    pub RandomSeed: LongTag,
    pub SpawnX: IntTag,
    pub SpawnY: IntTag,
    pub SpawnZ: IntTag,
    pub Time: LongTag,
    pub Player: Player,
}
