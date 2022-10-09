use crate::core::{CubeElement, CubeLoader, CubeModel};
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Actions {
    U,
    UC,
    D,
    DC,
    F,
    FC,
    B,
    BC,
    R,
    RC,
    L,
    LC,
}

pub trait Actionable {
    fn apply_action(&mut self, action: Actions);
}

impl Actionable for CubeModel {
    fn apply_action(&mut self, action: Actions) {
        match action {
            Actions::U => self.up_clockwise(),
            Actions::UC => self.up_counter_clockwise(),
            Actions::D => self.down_clockwise(),
            Actions::DC => self.down_counter_clockwise(),
            Actions::F => self.front_clockwise(),
            Actions::FC => self.front_counter_clockwise(),
            Actions::B => self.back_clockwise(),
            Actions::BC => self.back_counter_clockwise(),
            Actions::R => self.right_clockwise(),
            Actions::RC => self.right_counter_clockwise(),
            Actions::L => self.left_clockwise(),
            Actions::LC => self.left_counter_clockwise(),
        }
    }
}
