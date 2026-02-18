use crate::core::{state::{card::CardId, game::GameState, player::PlayerRole}, types::{command::Command, effect::{Effect, EffectContext}}};

pub(crate) fn expand_effect(
    _state: &GameState,
    controller: PlayerRole,
    _source: CardId,
    effect: Effect,
    _ctx: EffectContext,
) -> Vec<Command> {
    match effect {
        Effect::Draw { amount } => {
            vec![Command::DrawCards {
                player: controller,
                amount,
            }]
        }
        Effect::CancelEvent => todo!(),
        _ => todo!()
    }
}