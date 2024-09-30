// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_address_type::UniqueAddress;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `unique_address`.
///
/// Obtain a handle from the [`UniqueAddressTableAccess::unique_address`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_address()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_address().on_insert(...)`.
pub struct UniqueAddressTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueAddress>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_address`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueAddressTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueAddressTableHandle`], which mediates access to the table `unique_address`.
    fn unique_address(&self) -> UniqueAddressTableHandle<'_>;
}

impl UniqueAddressTableAccess for super::RemoteTables {
    fn unique_address(&self) -> UniqueAddressTableHandle<'_> {
        UniqueAddressTableHandle {
            imp: self.imp.get_table::<UniqueAddress>("unique_address"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueAddressInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueAddressDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueAddressTableHandle<'ctx> {
    type Row = UniqueAddress;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueAddress> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueAddressInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueAddressInsertCallbackId {
        UniqueAddressInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueAddressInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueAddressDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueAddressDeleteCallbackId {
        UniqueAddressDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueAddressDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueAddress>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"unique_address\"")
}

/// Access to the `a` unique index on the table `unique_address`,
/// which allows point queries on the field of the same name
/// via the [`UniqueAddressAUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_address().a().find(...)`.
pub struct UniqueAddressAUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueAddress, __sdk::Address>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueAddressTableHandle<'ctx> {
    /// Get a handle on the `a` unique index on the table `unique_address`.
    pub fn a(&self) -> UniqueAddressAUnique<'ctx> {
        UniqueAddressAUnique {
            imp: self.imp.get_unique_constraint::<__sdk::Address>("a", |row| &row.a),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueAddressAUnique<'ctx> {
    /// Find the subscribed row whose `a` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sdk::Address) -> Option<UniqueAddress> {
        self.imp.find(col_val)
    }
}
