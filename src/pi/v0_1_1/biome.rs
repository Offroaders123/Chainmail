use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BiomeResource {
    Rainforest = 0,
    Swampland,
    SeasonalForest,
    Savanna,
    Shrubland,
    Taiga,
    Plains,
    IceDesert,
    Tundra,
}
