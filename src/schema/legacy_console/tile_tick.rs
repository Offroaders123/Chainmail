use serde::{Deserialize, Serialize};

use crate::nbt::tag::StringTag;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TileTick {
    i: StringTag<TileTickResource>,
    // and other fields which are not yet known...
}

// Not sure the values for this yet, might be the same as Java
#[derive(Serialize, Deserialize)]
pub enum TileTickResource {}
