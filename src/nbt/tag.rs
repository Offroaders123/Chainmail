use std::collections::HashMap;

pub enum Tag {
    Byte(ByteTag),
    Short(ShortTag),
    Int(IntTag),
    Long(LongTag),
    Float(FloatTag),
    Double(DoubleTag),
    ByteArray(ByteArrayTag),
    String(StringTag),
    List(ListTag<Tag>),
    Compound(CompoundTag),
    IntArray(IntArrayTag),
    LongArray(LongArrayTag),
}

impl TagLike for Tag {}

pub struct ByteTag(i8);
pub struct ShortTag(i16);
pub struct IntTag(i32);
pub struct LongTag(i64);
pub struct FloatTag(f32);
pub struct DoubleTag(f64);
pub struct ByteArrayTag(Vec<i8>);
pub struct StringTag(String);
pub struct ListTag<T: TagLike>(Vec<T>);
pub struct CompoundTag(HashMap<String, Tag>);
pub struct IntArrayTag(Vec<i32>);
pub struct LongArrayTag(Vec<i64>);

pub trait TagLike {}

impl TagLike for ByteTag {}
impl TagLike for ShortTag {}
impl TagLike for IntTag {}
impl TagLike for LongTag {}
impl TagLike for FloatTag {}
impl TagLike for DoubleTag {}
impl TagLike for ByteArrayTag {}
impl TagLike for StringTag {}
impl<T: TagLike> TagLike for ListTag<T> {}
impl TagLike for CompoundTag {}
impl TagLike for IntArrayTag {}
impl TagLike for LongArrayTag {}
