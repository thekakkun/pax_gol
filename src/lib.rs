#![allow(unused_imports)]

pub mod board;
pub mod gol;

use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::primitives::{Group, Text};
use pax_std::types::StackerDirection;

use crate::gol::GameOfLife;

#[derive(Pax)]
#[main]
#[inlined(
    <Stacker cells=2 direction=StackerDirection::Vertical>
        <Text text="Game of Life, powered by Pax!" />
        <GameOfLife rows=5 cols=5 />
    </Stacker>
)]
pub struct App {}
