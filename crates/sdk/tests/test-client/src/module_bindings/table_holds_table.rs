// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::one_u_8::OneU8;
use super::vec_u_8::VecU8;
#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TableHoldsTable {
    pub a: OneU8,
    pub b: VecU8,
}

impl TableType for TableHoldsTable {
    const TABLE_NAME: &'static str = "TableHoldsTable";
    type ReducerEvent = super::ReducerEvent;
}

impl TableHoldsTable {
    #[allow(unused)]
    pub fn filter_by_a(a: OneU8) -> TableIter<Self> {
        Self::filter(|row| row.a == a)
    }
    #[allow(unused)]
    pub fn filter_by_b(b: VecU8) -> TableIter<Self> {
        Self::filter(|row| row.b == b)
    }
}