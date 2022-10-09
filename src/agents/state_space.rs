use super::action::{Actionable, Actions};
use super::heuristic::Heuristic;
use crate::core::{CubeElement, CubeLoader, CubeModel};
use core::mem::size_of;
use enum_iterator::all;
use std::{cmp::Reverse, collections::BinaryHeap};

// const STATE_SPACE_MEMORY_BOUND: f64 = 1e4;
const STATE_SPACE_BOUND: f64 = 1e6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeState {
    pub model: CubeModel,
    path_cost: Option<usize>,
    heuristic_cost: Option<usize>,
    actions_taken: Vec<Actions>,
    visited: bool,
}

impl CubeState {
    pub fn from_model(model: CubeModel) -> Self {
        Self {
            model,
            path_cost: None,
            heuristic_cost: None,
            actions_taken: Vec::new(),
            visited: false,
        }
    }

    pub fn elements(&self) -> &[[[CubeElement; 3]; 3]; 3] {
        self.model.cube_elements()
    }

    pub fn from_loader(loader: impl CubeLoader) -> Self {
        Self {
            model: CubeModel::from_loader(loader),
            path_cost: None,
            heuristic_cost: None,
            actions_taken: Vec::new(),
            visited: false,
        }
    }

    fn actions(&self) -> Vec<Actions> {
        all::<Actions>().collect()
    }

    fn set_visited(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }

    fn reset_visited(&mut self) {
        self.visited = false;
    }

    fn set_path_cost(&mut self, cost: usize) {
        self.path_cost = Some(cost);
    }

    fn set_heuristic_cost(&mut self, cost: usize) {
        self.heuristic_cost = Some(cost);
    }

    fn update_heristic_cost(&mut self) {
        self.set_heuristic_cost(self.heuristic());
    }
}

impl PartialOrd for CubeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_cost = self.path_cost? + self.heuristic_cost?;
        // let self_cost = self.heuristic_cost?;
        let other_cost = other.path_cost? + other.heuristic_cost?;
        // let other_cost = other.heuristic_cost?;
        Some(self_cost.cmp(&other_cost))
    }
}

impl Ord for CubeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Actionable for CubeState {
    fn apply_action(&mut self, action: Actions) {
        self.model.apply_action(action);
        self.actions_taken.push(action);
    }
}

pub struct StateSpace {
    goal_state: CubeState,
    frontier: BinaryHeap<Reverse<CubeState>>,
}

impl StateSpace {
    pub fn new(initial_state: CubeState, goal_state: CubeState) -> Self {
        let mut heap = BinaryHeap::new();
        let mut root = initial_state.clone();
        root.set_path_cost(0);
        root.set_heuristic_cost(100);
        heap.push(Reverse(root));

        Self {
            goal_state,
            frontier: heap,
        }
    }

    fn add_to_frontier(&mut self, state: CubeState) {
        self.frontier.push(Reverse(state));
    }

    fn prune_frontier(&mut self) {
        // let mut new_frontier = BinaryHeap::new();
        // let mut visited = Vec::new();
        // let state_bound = STATE_SPACE_MEMORY_BOUND / size_of::<CubeState>() as f64;

        if self.frontier.len() > (STATE_SPACE_BOUND * 2.0) as usize {
            let state_bound = STATE_SPACE_BOUND;
            let mut state_remaining = state_bound as usize;
            let old_front = self.frontier.clone();
            self.frontier = BinaryHeap::with_capacity(state_remaining);
            old_front.into_sorted_vec()
                .iter()
                .rev()
                .for_each(|Reverse(state)| {
                    if state_remaining > 0 {
                        self.frontier.push(Reverse(state.clone()));
                        state_remaining -= 1;
                    }
                });
        }
        // while let Some(Reverse(state)) = self.frontier.pop() {
        //     if !visited.contains(&state) {
        //         visited.push(state.clone());
        //         new_frontier.push(Reverse(state));
        //     }
        //     if state_remaining == 0 {
        //         break;
        //     }
        //     state_remaining -= 1;
        // }
        // self.frontier = new_frontier;
    }

    fn is_goal(&self, state: &CubeState) -> bool {
        state == &self.goal_state
    }

    fn get_successors(&self, state: &CubeState) -> Vec<CubeState> {
        state
            .actions()
            .iter()
            .map(|action| {
                let mut new_state = state.clone();
                new_state.apply_action(*action);
                new_state
            })
            .collect()
    }

    fn expand_frontier(&mut self) {
        // let mut new_frontier = BinaryHeap::new();
        let Reverse(promising_s) = self.frontier.pop().unwrap();
        let successors = self.get_successors(&promising_s);
        let promising_s_path_cost = promising_s.path_cost.unwrap();
        for successor in successors {
            let mut new_state = successor.clone();
            new_state.set_path_cost(promising_s_path_cost + 1);
            new_state.update_heristic_cost();
            self.add_to_frontier(new_state);
        }
    }

    pub fn solve(&mut self) -> Option<Vec<Actions>> {
        while !self.frontier.is_empty() {
            self.prune_frontier();
            self.expand_frontier();
            println!("Frontier size: {}", self.frontier.len());
            let Reverse(min_s) = self.frontier.peek().unwrap();
            println!("Frontier min: {}-{}", min_s.path_cost.unwrap(), min_s.heuristic_cost.unwrap());
            println!("Contour: {}", min_s.path_cost.unwrap() + min_s.heuristic_cost.unwrap());
            if let Some(Reverse(state)) = self.frontier.peek() {
                if state.heuristic_cost.unwrap() == 0 {
                    return Some(state.actions_taken.clone());
                }
            }
        }
        None
    }
}
