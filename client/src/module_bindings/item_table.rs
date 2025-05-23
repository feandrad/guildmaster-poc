// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::item_type::Item;
use spacetimedb_sdk::__codegen::{self as __sdk, __ws};

/// Table handle for the table `item`.
///
/// Obtain a handle from the [`ItemTableAccess::item`] method on [`super::RemoteTables`],
/// like `ctx.db.item()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.item().on_insert(...)`.
pub struct ItemTableHandle<'ctx> {
    imp: __sdk::TableHandle<Item>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `item`.
///
/// Implemented for [`super::RemoteTables`].
pub trait ItemTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`ItemTableHandle`], which mediates access to the table `item`.
    fn item(&self) -> ItemTableHandle<'_>;
}

impl ItemTableAccess for super::RemoteTables {
    fn item(&self) -> ItemTableHandle<'_> {
        ItemTableHandle {
            imp: self.imp.get_table::<Item>("item"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct ItemInsertCallbackId(__sdk::CallbackId);
pub struct ItemDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for ItemTableHandle<'ctx> {
    type Row = Item;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = Item> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = ItemInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> ItemInsertCallbackId {
        ItemInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: ItemInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = ItemDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> ItemDeleteCallbackId {
        ItemDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: ItemDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<Item>("item");
    _table.add_unique_constraint::<u32>("entity_id", |row| &row.entity_id);
}
pub struct ItemUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for ItemTableHandle<'ctx> {
    type UpdateCallbackId = ItemUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> ItemUpdateCallbackId {
        ItemUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: ItemUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<Item>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<Item>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `entity_id` unique index on the table `item`,
/// which allows point queries on the field of the same name
/// via the [`ItemEntityIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.item().entity_id().find(...)`.
pub struct ItemEntityIdUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<Item, u32>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> ItemTableHandle<'ctx> {
    /// Get a handle on the `entity_id` unique index on the table `item`.
    pub fn entity_id(&self) -> ItemEntityIdUnique<'ctx> {
        ItemEntityIdUnique {
            imp: self.imp.get_unique_constraint::<u32>("entity_id"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> ItemEntityIdUnique<'ctx> {
    /// Find the subscribed row whose `entity_id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u32) -> Option<Item> {
        self.imp.find(col_val)
    }
}
