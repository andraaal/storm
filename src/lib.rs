#![allow(dead_code)]
#![feature(checked_type_aliases)]

pub(crate) mod cards;
pub(crate) mod context;
pub(crate) mod game;
pub(crate) mod mana;
pub(crate) mod nested_borrow;
pub(crate) mod rules;

pub(crate) fn create_test_game() -> game::Game {
    todo!()
}
