pub trait ConvertTo<T> {
    fn convert(&self) -> T;
}

pub trait TryConvertTo<T> {
    fn try_convert(&self) -> Option<T>;
}
