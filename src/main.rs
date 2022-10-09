mod agents;
mod core;

use crate::agents::{Actions, Actionable};
use crate::agents::{CubeState, StateSpace};
use crate::core::{CubeModel, CubeTextLoader};
use enum_iterator::all;
use rand::prelude::*;
use std::path::PathBuf;

fn make_a_100_random_moves_model() -> CubeModel {
    let mut cube = CubeModel::solved();
    for _ in 0..10 {
        let random_move = all::<Actions>()
            .collect::<Vec<Actions>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned();
        match random_move {
            Actions::U => cube.up_clockwise(),
            Actions::UC => cube.up_counter_clockwise(),
            Actions::D => cube.down_clockwise(),
            Actions::DC => cube.down_counter_clockwise(),
            Actions::F => cube.front_clockwise(),
            Actions::FC => cube.front_counter_clockwise(),
            Actions::B => cube.back_clockwise(),
            Actions::BC => cube.back_counter_clockwise(),
            Actions::R => cube.right_clockwise(),
            Actions::RC => cube.right_counter_clockwise(),
            Actions::L => cube.left_clockwise(),
            Actions::LC => cube.left_counter_clockwise(),
        }
    }
    cube
}

fn main() {
    // let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //     .join("assets")
    //     .join("unsolved_cube.txt");
    // let loader = CubeTextLoader::from_file(path.as_path());
    let mut model = make_a_100_random_moves_model();
    let mut state_space = StateSpace::new(
        CubeState::from_model(model.to_owned()),
        CubeState::from_model(CubeModel::solved()),
    );
    let actions = state_space.solve();
    println!("Solved in {} steps", actions.as_ref().unwrap().len());
    println!("from:\n{}", model);
    for action in actions.unwrap() {
        println!("{:?}", action);
        model.apply_action(action);
    }
    println!("\nto:\n{}", model);
    println!("is solved: {}", model.is_solved());
}
