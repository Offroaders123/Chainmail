use std::collections::HashMap;

pub struct ByteTag(i8);
pub struct ShortTag(i16);
pub struct IntTag(i32);
pub struct LongTag(i64);
pub struct FloatTag(f32);
pub struct DoubleTag(f64);
pub struct ByteArrayTag(Vec<i8>);
pub struct StringTag(String);
pub struct ListTag<T: Tag>(Vec<T>);
pub struct CompoundTag(HashMap<String, Tag>);
pub struct IntArrayTag(Vec<i32>);
pub struct LongArrayTag(Vec<i64>);

pub trait Tag {}

impl Tag for ByteTag {}
impl Tag for ShortTag {}
impl Tag for IntTag {}
impl Tag for LongTag {}
impl Tag for FloatTag {}
impl Tag for DoubleTag {}
impl Tag for ByteArrayTag {}
impl Tag for StringTag {}
impl<T: Tag> Tag for ListTag<T> {}
impl Tag for CompoundTag {}
impl Tag for IntArrayTag {}
impl Tag for LongArrayTag {}
