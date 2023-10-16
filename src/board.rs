use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::primitives::Rectangle;
use pax_std::types::StackerDirection;
use rand::Rng;

#[derive(Pax)]
#[inlined(
    <Stacker cells={rows} direction=StackerDirection::Vertical>
    for row in 0..self.rows {
        <Stacker cells={cols} direction=StackerDirection::Horizontal>
        for col in 0..self.cols {
            <Rectangle width=10px height=10px
                fill={Fill::Solid(Color::rgb(1.0, 0.0, 0.0))}
            />
        }
        </Stacker>
        }
    </Stacker>

    @handlers {
        did_mount: handle_did_mount,
    }
)]
pub struct Board {
    pub rows: Property<u32>,
    pub cols: Property<u32>,
    pub state: Property<Vec<bool>>,
}
impl Board {
    pub fn handle_did_mount(&mut self, _ctx: RuntimeContext) {
        self.state.set(vec![
            false;
            (self.rows.get() * self.cols.get()).try_into().unwrap()
        ]);
        println!("{:?}", self.state.get())
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.state.get_mut().iter_mut() {
            *cell = rng.gen::<bool>();
        }
    }
}
