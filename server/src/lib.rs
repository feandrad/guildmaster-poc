use spacetimedb::ReducerContext;

pub mod constants;
pub mod models;
pub mod entities;
pub mod events;
pub mod systems;
pub mod maps;
pub mod math;

#[spacetimedb::reducer]
pub fn debug(ctx: &ReducerContext) -> Result<(), String> {
    log::debug!("This reducer was called by {}.", ctx.sender);
    Ok(())
}