use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::primitives::{Group, Text};
use pax_std::types::StackerDirection;
use rand::Rng;

use crate::board::Board;

#[derive(Pax)]
#[inlined(
    <Stacker cells=2 direction=StackerDirection::Vertical>
        <Board></Board>
    </Stacker>

    @handlers {
        did_mount: handle_did_mount,
    }
)]
pub struct GameOfLife {
    pub rows: Property<u32>,
    pub cols: Property<u32>,
    pub world: Property<Vec<bool>>,
}

impl GameOfLife {
    pub fn handle_did_mount(&mut self, _ctx: RuntimeContext) {
        self.world.set(vec![
            false;
            (self.rows.get() * self.cols.get()).try_into().unwrap()
        ]);
        println!("{:?}", self.world.get())
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.world.get_mut().iter_mut() {
            *cell = rng.gen::<bool>();
        }
    }

    pub fn tick(&mut self) {}
}
