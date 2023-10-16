use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::types::StackerDirection;

#[derive(Pax)]
#[inlined(
    <Stacker cells=5 direction=StackerDirection::Vertical>
    for row in 0..5 {
        <Stacker cells=5 direction=StackerDirection::Horizontal>
        for col in 0..5 {
            <Rectangle width=10px height=10px
                fill={Fill::Solid(Color::rgb(1.0, 0.0, 0.0))}
            />
        }
        </Stacker>
        }
    </Stacker>
)]
pub struct Board {}
