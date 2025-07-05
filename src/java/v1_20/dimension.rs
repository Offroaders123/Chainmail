use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionResource {
    the_nether,
    overworld,
    the_end,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionID {
    the_nether = -1,
    overworld,
    the_end,
}
