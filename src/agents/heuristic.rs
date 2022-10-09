use super::state_space::CubeState;
use crate::core::SOLVED_INDEX_MAP;

trait ExpectedMoves {
    fn expected_move_distance(&self) -> f64;
}

impl ExpectedMoves for CubeState {
    fn expected_move_distance(&self) -> f64 {
        let mut distance = 0.;
        let elems = self.model.cube_elements();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let index = (i, j, k);
                    let cubie = elems[i][j][k];
                    // get the index of cubie in SOLVED_INDEX_MAP
                    let solved_index = SOLVED_INDEX_MAP
                        .iter()
                        .filter(|(_, x)| *x == cubie)
                        .map(|(x, _)| x)
                        .next()
                        .unwrap();
                    // calculate the distance
                    let mut temp_distance = 0;
                    let (x, y, z) = *solved_index;
                    // handles the case where the cubie is in the right place
                    if index == (x, y, z) {
                        continue;
                    }
                    // handles the cases of vertex and edge cubies
                    if x != 1 && y != 1 && z != 1 {        // vertex cubie
                        if i != x && j != y && k != z {
                            temp_distance += 3;
                        } else if i == x && j == y || i == x && k == z || j == y && k == z {
                            temp_distance += 1;
                        } else {
                            temp_distance += 2;
                        }
                    } else if x == 1 || y == 1 || z == 1 { // edge cubie
                        if i == x && j == y  || i == x && k == z || j == y && k == z {
                            temp_distance += 2;
                        } else if i == x && j != y && k != z || j == y && i != x && k != z || k == z && i != x && j != y {
                            temp_distance += 4;
                        } else {
                            temp_distance += 1;
                        }
                    }
                    distance += temp_distance as f64 / 8.0;
                }
            }
        }
        distance
    }
}

pub trait ManhattanDistance {
    fn manhattan_distance(&self) -> f64;
}

impl ManhattanDistance for CubeState {
    fn manhattan_distance(&self) -> f64 {
        let mut distance = 0.;
        let elems = self.model.cube_elements();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let index = (i, j, k);
                    let cubie = elems[i][j][k];
                    // get the index of cubie in SOLVED_INDEX_MAP
                    let solved_index = SOLVED_INDEX_MAP
                        .iter()
                        .filter(|(_, x)| *x == cubie)
                        .map(|(x, _)| x)
                        .next()
                        .unwrap();
                    // calculate the manhattan distance
                    let mut vertex_distance = 0;
                    let mut edge_distance = 0;
                    let (x, y, z) = *solved_index;
                    // handles the case where the cubie is in the right place
                    if index == (x, y, z) {
                        continue;
                    }
                    // handles the cases of vertex and edge cubies
                    if x != 1 && y != 1 && z != 1 {        // vertex cubie
                        vertex_distance += (i as i32 - x as i32).abs();
                        vertex_distance += (j as i32 - y as i32).abs();
                        vertex_distance += (k as i32 - z as i32).abs();
                    } else if x == 1 || y == 1 || z == 1 { // edge cubie
                        edge_distance += (i as i32 - x as i32).abs();
                        edge_distance += (j as i32 - y as i32).abs();
                        edge_distance += (k as i32 - z as i32).abs();
                    }
                    distance += vertex_distance as f64 / 4.0 + edge_distance as f64 / 4.0;
                }
            }
        }
        distance
    }
}

pub trait Heuristic {
    fn heuristic(&self) -> f64;
}

impl Heuristic for CubeState {
    fn heuristic(&self) -> f64 {
        // self.expected_move_distance()
        // self.manhattan_distance()
        self.manhattan_distance().max(self.expected_move_distance())
    }
}