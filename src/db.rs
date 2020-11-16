use crate::Error;
use rusqlite::params;
use rusqlite::types::Type;
use rusqlite::{Connection, ToSql};
use serenity::static_assertions::_core::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;

pub trait DbObject {
    type Column: Column;

    const TABLE_NAME: &'static str;

    fn values(&self) -> HashMap<Self::Column, &dyn ToSql>;

    fn id(&self) -> Option<i64>;

    fn create_table(db: &Connection) -> Result<(), Error> {
        let mut columns = Self::Column::all_columns().into_iter();

        let columns_str = match columns.next() {
            Some(col) => {
                let first_col = format!("{} {}", col, col.ty());
                columns.fold(first_col, |cols, col| {
                    format!("{}, {} {}", cols, col, col.ty())
                })
            }
            None => String::new(),
        };

        db.execute(
            &format!("CREATE TABLE {} ({})", Self::TABLE_NAME, columns_str),
            params![],
        )?;

        Ok(())
    }
}

pub trait Column: PartialEq + Eq + Hash + Display + Sized {
    fn all_columns() -> Vec<Self>;

    fn ty(&self) -> Type;
}
