use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Boolean(BooleanTag),
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

pub type BooleanTag = bool;
pub type ByteTag<T: Into<i8> = i8> = T;
pub type ShortTag<T: Into<i16> = i16> = T;
pub type IntTag<T: Into<i32> = i32> = T;
pub type LongTag = i64;
pub type FloatTag<T: Into<f32> = f32> = T;
pub type DoubleTag = f64;
#[derive(Serialize, Deserialize)]
pub struct ByteArrayTag(Vec<i8>);
pub type StringTag<T: Into<String> = String> = T;
pub type ListTag<T: TagLike> = Vec<T>;
pub type CompoundTag<T: TagLike = Tag> = HashMap<String, T>;
#[derive(Serialize, Deserialize)]
pub struct IntArrayTag(Vec<i32>);
#[derive(Serialize, Deserialize)]
pub struct LongArrayTag(Vec<i64>);

impl TagLike for Tag {}
impl TagLike for BooleanTag {}
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
