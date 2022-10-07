use crate::core::{CubeElement, CubeLoader, CubeModel};
use enum_iterator::{all, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Sequence)]
enum Actions {
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

trait Actionable {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct CubeState {
    model: CubeModel,
}

impl CubeState {
    fn from_model(model: CubeModel) -> Self {
        Self { model }
    }

    fn from_loader(loader: impl CubeLoader) -> Self {
        Self {
            model: CubeModel::from_loader(loader),
        }
    }

    fn actions(&self) -> Vec<Actions> {
        all::<Actions>().collect()
    }
}

impl Actionable for CubeState {
    fn apply_action(&mut self, action: Actions) {
        self.model.apply_action(action);
    }
}
