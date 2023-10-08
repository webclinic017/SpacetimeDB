// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

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
pub struct InsertVecU128Args {
    pub n: Vec<u128>,
}

impl Reducer for InsertVecU128Args {
    const REDUCER_NAME: &'static str = "insert_vec_u128";
}

#[allow(unused)]
pub fn insert_vec_u_128(n: Vec<u128>) {
    InsertVecU128Args { n }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_u_128(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<u128>) + Send + 'static,
) -> ReducerCallbackId<InsertVecU128Args> {
    InsertVecU128Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecU128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_u_128(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<u128>) + Send + 'static,
) -> ReducerCallbackId<InsertVecU128Args> {
    InsertVecU128Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecU128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_u_128(id: ReducerCallbackId<InsertVecU128Args>) {
    InsertVecU128Args::remove_on_reducer(id);
}