use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::types::StackerDirection;

use crate::board::Board;

#[derive(Pax)]
#[inlined(
    <Stacker cells=2 direction=StackerDirection::Vertical>
        <Board rows=5 cols=5 />
    </Stacker>
)]
pub struct GameOfLife {}
impl GameOfLife {
    pub fn start(&mut self) {
        todo!()
    }

    pub fn step(&mut self) {
        todo!()
    }

    pub fn pause(&mut self) {
        todo!()
    }
}
