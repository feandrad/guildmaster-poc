use spacetimedb::ReducerContext;

pub mod constants;
pub mod models;
pub mod entities;
pub mod events;
pub mod systems;
pub mod maps;
pub mod math;

#[spacetimedb::reducer(client_connected)]
pub fn connect(ctx: &ReducerContext) -> Result<(), String> {
    log::debug!("{} just connected.", ctx.sender);
    Ok(())
}