use cardgame_core::core::state::card::CardDef;

pub struct CardRegistration(pub fn() -> CardDef);

pub mod cards;

inventory::collect!(CardRegistration);

pub fn all_card_defs() -> Vec<CardDef> {
    inventory::iter::<CardRegistration>
        .into_iter()
        .map(|reg| (reg.0)())
        .collect()
}
