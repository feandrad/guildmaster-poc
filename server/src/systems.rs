use crate::models::{Player, ChatMessage};
use crate::events::GameEvent;
use spacetime::{HandlerContext, SpacetimeResult};

#[spacetime_handler]
pub fn on_spawn(event: &GameEvent, ctx: &mut HandlerContext) -> SpacetimeResult<()> {
    if let GameEvent::SpawnPlayer { player_id, name, color, shape } = event {
        ctx.insert(Player {
            id: player_id.clone(),
            name: name.clone(),
            color: color.clone(),
            shape: shape.clone(),
            x: 0,
            y: 0,
            map_id: "village".to_string(),
        })?;
    }
    Ok(())
}

#[spacetime_handler]
pub fn on_move(event: &GameEvent, ctx: &mut HandlerContext) -> SpacetimeResult<()> {
    if let GameEvent::Move { player_id, dx, dy } = event {
        let mut player = ctx.entity_mut::<Player>(player_id)?;
        player.x += dx;
        player.y += dy;

        // Verificar transições de mapa (exemplo simplificado)
        if player.x > 100 {
            player.map_id = "forest".to_string();
            player.x = 0;
            player.y = 0;
        }
    }
    Ok(())
}

#[spacetime_handler]
pub fn on_chat(event: &GameEvent, ctx: &mut HandlerContext) -> SpacetimeResult<()> {
    if let GameEvent::ChatMessage { player_id, text } = event {
        let player = ctx.entity::<Player>(player_id)?;
        ctx.insert(ChatMessage {
            id: ctx.next_id()?,
            sender_id: player_id.clone(),
            text: text.clone(),
            map_id: player.map_id.clone(),
            timestamp: ctx.timestamp(),
        })?;
    }
    Ok(())
}
