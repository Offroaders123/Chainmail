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

pub trait TagLike {}

pub type ByteTag = i8;
pub type ShortTag<T: Into<i16> = i16> = T;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
pub struct ByteArrayTag(Vec<i8>);
pub type StringTag = String;
pub type ListTag<T: TagLike> = Vec<T>;
pub type CompoundTag = HashMap<String, Tag>;
pub struct IntArrayTag(Vec<i32>);
pub struct LongArrayTag(Vec<i64>);

impl TagLike for Tag {}
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
