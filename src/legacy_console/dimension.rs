use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionID {
    nether = -1,
    overworld,
    the_end,
}
