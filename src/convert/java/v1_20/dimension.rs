use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::dimension as java, pi::v0_1_1::dimension as pi},
};

impl ConvertTo<pi::DimensionID> for java::DimensionResource {
    fn convert(&self) -> pi::DimensionID {
        match self {
            Self::the_nether => pi::DimensionID::nether,
            Self::overworld => pi::DimensionID::overworld,
            Self::the_end => pi::DimensionID::the_end,
        }
    }
}
