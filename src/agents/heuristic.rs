use super::state_space::CubeState;
use crate::core::SOLVED_INDEX_MAP;

trait MoveDistance {
    fn move_distance(&self) -> usize;
}

trait MisplacedTiles {
    fn misplaced_tiles(&self) -> usize;
}

trait ColorDistance {
    fn color_distance(&self) -> usize;
}

pub trait Heuristic {
    fn heuristic(&self) -> usize;
}

impl MoveDistance for CubeState {
    fn move_distance(&self) -> usize {
        let elements = self.elements();
        let mut distance = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let element = elements[i][j][k];
                    let index = SOLVED_INDEX_MAP.iter().filter(|(_, e)| *e == element).next().unwrap().0;
                    let (l, m, n) = index;
                    distance += (i as isize - l as isize).abs() as usize;
                    distance += (j as isize - m as isize).abs() as usize;
                    distance += (k as isize - n as isize).abs() as usize;
                }
            }
        }
        distance
    }
}

impl MisplacedTiles for CubeState {
    fn misplaced_tiles(&self) -> usize {
        let elements = self.elements();
        let mut distance = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let element = elements[i][j][k];
                    let index = SOLVED_INDEX_MAP.iter().filter(|(_, e)| *e == element).next().unwrap().0;
                    let (l, m, n) = index;
                    if i != l || j != m || k != n {
                        distance += 1;
                    }
                }
            }
        }
        distance
    }
}

impl ColorDistance for CubeState {
    fn color_distance(&self) -> usize {
        let elements = self.elements();
        let mut distance = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let element = elements[i][j][k];
                    let index = SOLVED_INDEX_MAP.iter().filter(|(_, e)| *e == element).next().unwrap().0;
                    let (l, m, n) = index;
                    if i != l {
                        distance += 1;
                    }
                    if j != m {
                        distance += 1;
                    }
                    if k != n {
                        distance += 1;
                    }
                }
            }
        }
        distance
    }
}

trait ColorDisparity {
    fn color_disparity(&self) -> usize;
}

impl ColorDisparity for CubeState {
    fn color_disparity(&self) -> usize {
        let elements = self.elements();
        let mut same_color_neighbors = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let element = elements[i][j][k];
                    let colors = element.to_tile_colors();
                    // check if the element has neighbors with the same color
                    if i > 0 {
                        let other_elements = elements[i - 1][j][k];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                    if i < 2 {
                        let other_elements = elements[i + 1][j][k];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                    if j > 0 {
                        let other_elements = elements[i][j - 1][k];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                    if j < 2 {
                        let other_elements = elements[i][j + 1][k];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                    if k > 0 {
                        let other_elements = elements[i][j][k - 1];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                    if k < 2 {
                        let other_elements = elements[i][j][k + 1];
                        let other_colors = other_elements.to_tile_colors();
                        for color in colors.iter() {
                            if other_colors.contains(color) {
                                same_color_neighbors += 1;
                            }
                        }
                    }
                }
            }
        }
        same_color_neighbors
    }
}

impl Heuristic for CubeState {
    fn heuristic(&self) -> usize {
        // self.misplaced_tiles() + self.move_distance() + self.color_distance()
        // self.misplaced_tiles() + self.color_distance()
        self.color_disparity() + self.misplaced_tiles()
    }
}