use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::item as java, pi::v0_1_1::item as pi},
};

impl ConvertTo<pi::Item> for java::Item {
    fn convert(&self) -> pi::Item {
        pi::Item {
            id: self.id.convert(),
            Damage: self.tag.map_or(0, |tag| clamp_damage(tag.Damage)),
            Count: self.Count,
        }
    }
}

/// Converts a Java Edition item damage value (`i32`) to a Bedrock Edition-compatible value (`i16`).
///
/// In Java Edition, `Damage` is stored as a 32-bit signed integer. In Bedrock Edition,
/// it is stored as a 16-bit signed integer. To preserve vanilla-compatible behavior,
/// this function clamps the input to the valid `i16` range: -32,768 to 32,767.
///
/// This avoids overflow and wraparound artifacts that could result in undefined behavior
/// or corrupted item states in Bedrock.
///
/// # Parameters
/// - `value`: The Java Edition `Damage` value as `i32`.
///
/// # Returns
/// A clamped `i16` value safe for use in Bedrock Edition NBT.
///
/// # Example
/// ```
/// let bedrock_damage = clamp_damage(40000);
/// assert_eq!(bedrock_damage, 32767);
/// ```
fn clamp_damage(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}
