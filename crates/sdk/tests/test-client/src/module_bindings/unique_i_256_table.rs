// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_i_256_type::UniqueI256;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `unique_i256`.
///
/// Obtain a handle from the [`UniqueI256TableAccess::unique_i_256`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_i_256()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_256().on_insert(...)`.
pub struct UniqueI256TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueI256>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_i256`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueI256TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueI256TableHandle`], which mediates access to the table `unique_i256`.
    fn unique_i_256(&self) -> UniqueI256TableHandle<'_>;
}

impl UniqueI256TableAccess for super::RemoteTables {
    fn unique_i_256(&self) -> UniqueI256TableHandle<'_> {
        UniqueI256TableHandle {
            imp: self.imp.get_table::<UniqueI256>("unique_i256"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueI256InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueI256DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueI256TableHandle<'ctx> {
    type Row = UniqueI256;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueI256> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueI256InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI256InsertCallbackId {
        UniqueI256InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueI256InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueI256DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI256DeleteCallbackId {
        UniqueI256DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueI256DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueI256>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"unique_i256\"")
}

/// Access to the `n` unique index on the table `unique_i256`,
/// which allows point queries on the field of the same name
/// via the [`UniqueI256NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_256().n().find(...)`.
pub struct UniqueI256NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueI256, __sats::i256>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueI256TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_i256`.
    pub fn n(&self) -> UniqueI256NUnique<'ctx> {
        UniqueI256NUnique {
            imp: self.imp.get_unique_constraint::<__sats::i256>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueI256NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sats::i256) -> Option<UniqueI256> {
        self.imp.find(col_val)
    }
}
