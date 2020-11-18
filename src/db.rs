use crate::Error;
use rusqlite::types::Type;
use rusqlite::Connection;
use std::rc::Rc;
use std::sync::Arc;

pub trait DbObject {
    fn create_table(db: &Connection) -> Result<(), Error>;

    fn save(&self, db: &Connection) -> Result<(), Error>;

    fn reload(&mut self, db: &Connection) -> Result<&mut Self, Error>;

    fn destroy(&self, db: &Connection) -> Result<(), Error>;
}

pub trait DbValue {
    const TYPE: Type;
}

macro_rules! db_values {
    ($($ty:ty => $var:ident),*) => {
        $(
            impl DbValue for $ty {
                const TYPE: Type = Type::$var;
            }
        )*
    };
}

db_values! {
    bool => Integer,
    i8 => Integer,
    i16 => Integer,
    i32 => Integer,
    i64 => Integer,
    isize => Integer,
    u8 => Integer,
    u16 => Integer,
    u32 => Integer,
    f32 => Real,
    f64 => Real,
    String => Text,
    &str => Text,
    Vec<u8> => Blob,
    [u8] => Blob
}

impl<T: DbValue + ?Sized> DbValue for &T {
    const TYPE: Type = T::TYPE;
}

impl<T: DbValue> DbValue for Option<T> {
    const TYPE: Type = T::TYPE;
}

impl<T: DbValue + ?Sized> DbValue for Box<T> {
    const TYPE: Type = T::TYPE;
}

impl<T: DbValue + ?Sized> DbValue for Rc<T> {
    const TYPE: Type = T::TYPE;
}

impl<T: DbValue + ?Sized> DbValue for Arc<T> {
    const TYPE: Type = T::TYPE;
}
