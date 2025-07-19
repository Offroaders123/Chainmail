use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::item as java, pi::v0_1_1::item as pi},
};

impl ConvertTo<pi::Item> for java::Item {
    fn convert(&self) -> pi::Item {
        pi::Item {}
    }
}
