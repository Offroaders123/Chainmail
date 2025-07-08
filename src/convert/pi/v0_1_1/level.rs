use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::level as java, pi::v0_1_1::level as pi},
};

impl ConvertTo<java::LevelDat> for pi::LevelDat {
    fn convert(&self) -> java::LevelDat {}
}
