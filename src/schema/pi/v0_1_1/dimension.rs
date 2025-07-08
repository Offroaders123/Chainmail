use serde::{Deserialize, Serialize};

// Got these from the bedrock types, not sure if these are still the same in this old version
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionID {
    nether = -1,
    overworld,
    the_end,
}
