use serde::{Deserialize, Serialize};

// These should be `minecraft:` prefixed when serialized
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionResource {
    nether,
    overworld,
    the_end,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionID {
    overworld = 0,
    nether,
    end,
}
