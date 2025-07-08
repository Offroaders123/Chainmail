use crate::{
    convert::convert_to::ConvertTo,
    schema::{
        java::v1_20::level::LevelDat as JavaLevelDat, pi::v0_1_1::level::LevelDat as PiLevelDat,
    },
};

impl ConvertTo<PiLevelDat> for JavaLevelDat {
    fn convert(&self) -> PiLevelDat {}
}
