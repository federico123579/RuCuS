use std::fmt::Display;
use super::loaders::CubeLoader;

/// CubeModel is thought to model the Rubik's cube with (0,0,0) being the yellow-red-blue vertix.
/// The x-axis is from Red to Orange (left to right)
/// The y-axis is from Yellow to White (bottom to top)
/// The z-axis is from Blue to Green (front to back).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubeModel {
    cube_elements: [[[CubeElement; 3]; 3]; 3],
}

impl CubeModel {
    pub fn from_loader(loader: impl CubeLoader) -> Self {
        Self {
            cube_elements: loader.to_model_elements(),
        }
    }

    fn right_clockwise(&mut self) {
        let mut temp = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = temp;

        temp = self.cube_elements[2][0][1];
        self.cube_elements[2][0][1] = self.cube_elements[2][1][2];
        self.cube_elements[2][1][2] = self.cube_elements[2][2][1];
        self.cube_elements[2][2][1] = self.cube_elements[2][1][0];
        self.cube_elements[2][1][0] = temp;
    }

    fn right_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = temp;

        temp = self.cube_elements[2][0][1];
        self.cube_elements[2][0][1] = self.cube_elements[2][1][0];
        self.cube_elements[2][1][0] = self.cube_elements[2][2][1];
        self.cube_elements[2][2][1] = self.cube_elements[2][1][2];
        self.cube_elements[2][1][2] = temp;
    }

    fn left_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = temp;

        temp = self.cube_elements[0][0][1];
        self.cube_elements[0][0][1] = self.cube_elements[0][1][0];
        self.cube_elements[0][1][0] = self.cube_elements[0][2][1];
        self.cube_elements[0][2][1] = self.cube_elements[0][1][2];
        self.cube_elements[0][1][2] = temp;
    }

    fn left_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = temp;

        temp = self.cube_elements[0][0][1];
        self.cube_elements[0][0][1] = self.cube_elements[0][1][2];
        self.cube_elements[0][1][2] = self.cube_elements[0][2][1];
        self.cube_elements[0][2][1] = self.cube_elements[0][1][0];
        self.cube_elements[0][1][0] = temp;
    }

    fn top_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = temp;

        temp = self.cube_elements[0][2][1];
        self.cube_elements[0][2][1] = self.cube_elements[1][2][0];
        self.cube_elements[1][2][0] = self.cube_elements[2][2][1];
        self.cube_elements[2][2][1] = self.cube_elements[1][2][2];
        self.cube_elements[1][2][2] = temp;
    }

    fn top_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = temp;

        temp = self.cube_elements[0][2][1];
        self.cube_elements[0][2][1] = self.cube_elements[1][2][2];
        self.cube_elements[1][2][2] = self.cube_elements[2][2][1];
        self.cube_elements[2][2][1] = self.cube_elements[1][2][0];
        self.cube_elements[1][2][0] = temp;
    }

    fn bottom_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = temp;

        temp = self.cube_elements[0][0][1];
        self.cube_elements[0][0][1] = self.cube_elements[1][0][2];
        self.cube_elements[1][0][2] = self.cube_elements[2][0][1];
        self.cube_elements[2][0][1] = self.cube_elements[1][0][0];
        self.cube_elements[1][0][0] = temp;
    }

    fn bottom_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = temp;

        temp = self.cube_elements[0][0][1];
        self.cube_elements[0][0][1] = self.cube_elements[1][0][0];
        self.cube_elements[1][0][0] = self.cube_elements[2][0][1];
        self.cube_elements[2][0][1] = self.cube_elements[1][0][2];
        self.cube_elements[1][0][2] = temp;
    }

    fn front_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = temp;

        temp = self.cube_elements[0][1][0];
        self.cube_elements[0][1][0] = self.cube_elements[1][0][0];
        self.cube_elements[1][0][0] = self.cube_elements[2][1][0];
        self.cube_elements[2][1][0] = self.cube_elements[1][2][0];
        self.cube_elements[1][2][0] = temp;
    }

    fn front_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][0];
        self.cube_elements[0][0][0] = self.cube_elements[0][2][0];
        self.cube_elements[0][2][0] = self.cube_elements[2][2][0];
        self.cube_elements[2][2][0] = self.cube_elements[2][0][0];
        self.cube_elements[2][0][0] = temp;

        temp = self.cube_elements[0][1][0];
        self.cube_elements[0][1][0] = self.cube_elements[1][2][0];
        self.cube_elements[1][2][0] = self.cube_elements[2][1][0];
        self.cube_elements[2][1][0] = self.cube_elements[1][0][0];
        self.cube_elements[1][0][0] = temp;
    }

    fn back_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = temp;

        temp = self.cube_elements[0][1][2];
        self.cube_elements[0][1][2] = self.cube_elements[1][2][2];
        self.cube_elements[1][2][2] = self.cube_elements[2][1][2];
        self.cube_elements[2][1][2] = self.cube_elements[1][0][2];
        self.cube_elements[1][0][2] = temp;
    }

    fn back_counter_clockwise(&mut self) {
        let mut temp = self.cube_elements[0][0][2];
        self.cube_elements[0][0][2] = self.cube_elements[2][0][2];
        self.cube_elements[2][0][2] = self.cube_elements[2][2][2];
        self.cube_elements[2][2][2] = self.cube_elements[0][2][2];
        self.cube_elements[0][2][2] = temp;

        temp = self.cube_elements[0][1][2];
        self.cube_elements[0][1][2] = self.cube_elements[1][0][2];
        self.cube_elements[1][0][2] = self.cube_elements[2][1][2];
        self.cube_elements[2][1][2] = self.cube_elements[1][2][2];
        self.cube_elements[1][2][2] = temp;
    }
}

impl Display for CubeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let s = format!(
                        "{:4}",
                        self.cube_elements[i][j][k].to_string().replace("-", "")
                    );
                    result.push_str(&s);
                }
                result.push_str("\n");
            }
        }
        result.push_str("\n");
        f.write_str(&result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeElement {
    WhiteOrangeGreen,
    WhiteBlueOrange,
    WhiteGreenRed,
    WhiteRedBlue,
    YellowOrangeGreen,
    YellowBlueOrange,
    YellowGreenRed,
    YellowRedBlue,
    WhiteOrange,
    WhiteGreen,
    WhiteRed,
    WhiteBlue,
    YellowOrange,
    YellowGreen,
    YellowRed,
    YellowBlue,
    RedBlue,
    RedGreen,
    OrangeBlue,
    OrangeGreen,
    White,
    Orange,
    Green,
    Red,
    Blue,
    Yellow,
    Kernel,
}

impl Display for CubeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            CubeElement::WhiteOrange => "W-O",
            CubeElement::WhiteGreen => "W-G",
            CubeElement::WhiteRed => "W-R",
            CubeElement::WhiteBlue => "W-B",
            CubeElement::YellowOrange => "Y-O",
            CubeElement::YellowGreen => "Y-G",
            CubeElement::YellowRed => "Y-R",
            CubeElement::YellowBlue => "Y-B",
            CubeElement::RedBlue => "R-B",
            CubeElement::RedGreen => "R-G",
            CubeElement::OrangeBlue => "O-B",
            CubeElement::OrangeGreen => "O-G",
            CubeElement::White => "W",
            CubeElement::Orange => "O",
            CubeElement::Green => "G",
            CubeElement::Red => "R",
            CubeElement::Blue => "B",
            CubeElement::Yellow => "Y",
            CubeElement::Kernel => "K",
            CubeElement::WhiteOrangeGreen => "W-O-G",
            CubeElement::WhiteBlueOrange => "W-B-O",
            CubeElement::WhiteGreenRed => "W-G-R",
            CubeElement::WhiteRedBlue => "W-R-B",
            CubeElement::YellowOrangeGreen => "Y-O-G",
            CubeElement::YellowBlueOrange => "Y-B-O",
            CubeElement::YellowGreenRed => "Y-G-R",
            CubeElement::YellowRedBlue => "Y-R-B",
        };
        f.write_str(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::loaders::CubeTextLoader;
    use std::path::PathBuf;

    fn load_solved_cube() -> CubeModel {
        // get the path of the file
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("solved_cube.txt");
        let loader = CubeTextLoader::from_file(path.as_path());
        CubeModel::from_loader(loader)
    }

    mod actions {
        use super::*;

        #[test]
        fn right_clockwise() {
            let mut cube = load_solved_cube();
            cube.right_clockwise();
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::YellowOrangeGreen);
            assert_eq!(cube.cube_elements[2][0][1], CubeElement::OrangeGreen);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[2][1][0], CubeElement::YellowOrange);
            assert_eq!(cube.cube_elements[2][1][1], CubeElement::Orange);
            assert_eq!(cube.cube_elements[2][1][2], CubeElement::WhiteOrange);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::YellowBlueOrange);
            assert_eq!(cube.cube_elements[2][2][1], CubeElement::OrangeBlue);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::WhiteBlueOrange);
        }

        #[test]
        fn right_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.right_counter_clockwise();
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::WhiteBlueOrange);
            assert_eq!(cube.cube_elements[2][0][1], CubeElement::OrangeBlue);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::YellowBlueOrange);
            assert_eq!(cube.cube_elements[2][1][0], CubeElement::WhiteOrange);
            assert_eq!(cube.cube_elements[2][1][1], CubeElement::Orange);
            assert_eq!(cube.cube_elements[2][1][2], CubeElement::YellowOrange);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[2][2][1], CubeElement::OrangeGreen);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::YellowOrangeGreen);
        }

        #[test]
        fn left_clockwise() {
            let mut cube = load_solved_cube();
            cube.left_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::WhiteRedBlue);
            assert_eq!(cube.cube_elements[0][0][1], CubeElement::RedBlue);
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[0][1][0], CubeElement::WhiteRed);
            assert_eq!(cube.cube_elements[0][1][1], CubeElement::Red);
            assert_eq!(cube.cube_elements[0][1][2], CubeElement::YellowRed);
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::WhiteGreenRed);
            assert_eq!(cube.cube_elements[0][2][1], CubeElement::RedGreen);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::YellowGreenRed);
        }

        #[test]
        fn left_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.left_counter_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::YellowGreenRed);
            assert_eq!(cube.cube_elements[0][0][1], CubeElement::RedGreen);
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::WhiteGreenRed);
            assert_eq!(cube.cube_elements[0][1][0], CubeElement::YellowRed);
            assert_eq!(cube.cube_elements[0][1][1], CubeElement::Red);
            assert_eq!(cube.cube_elements[0][1][2], CubeElement::WhiteRed);
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[0][2][1], CubeElement::RedBlue);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::WhiteRedBlue);
        }

        #[test]
        fn top_clockwise() {
            let mut cube = load_solved_cube();
            cube.top_clockwise();
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::WhiteBlueOrange);
            assert_eq!(cube.cube_elements[0][2][1], CubeElement::WhiteBlue);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::WhiteRedBlue);
            assert_eq!(cube.cube_elements[1][2][0], CubeElement::WhiteOrange);
            assert_eq!(cube.cube_elements[1][2][1], CubeElement::White);
            assert_eq!(cube.cube_elements[1][2][2], CubeElement::WhiteRed);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[2][2][1], CubeElement::WhiteGreen);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::WhiteGreenRed);
        }

        #[test]
        fn top_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.top_counter_clockwise();
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::WhiteGreenRed);
            assert_eq!(cube.cube_elements[0][2][1], CubeElement::WhiteGreen);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[1][2][0], CubeElement::WhiteRed);
            assert_eq!(cube.cube_elements[1][2][1], CubeElement::White);
            assert_eq!(cube.cube_elements[1][2][2], CubeElement::WhiteOrange);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::WhiteRedBlue);
            assert_eq!(cube.cube_elements[2][2][1], CubeElement::WhiteBlue);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::WhiteBlueOrange);
        }

        #[test]
        fn bottom_clockwise() {
            let mut cube = load_solved_cube();
            cube.bottom_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::YellowGreenRed);
            assert_eq!(cube.cube_elements[0][0][1], CubeElement::YellowGreen);
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::YellowOrangeGreen);
            assert_eq!(cube.cube_elements[1][0][0], CubeElement::YellowRed);
            assert_eq!(cube.cube_elements[1][0][1], CubeElement::Yellow);
            assert_eq!(cube.cube_elements[1][0][2], CubeElement::YellowOrange);
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[2][0][1], CubeElement::YellowBlue);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::YellowBlueOrange);
        }

        #[test]
        fn bottom_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.bottom_counter_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::YellowBlueOrange);
            assert_eq!(cube.cube_elements[0][0][1], CubeElement::YellowBlue);
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[1][0][0], CubeElement::YellowOrange);
            assert_eq!(cube.cube_elements[1][0][1], CubeElement::Yellow);
            assert_eq!(cube.cube_elements[1][0][2], CubeElement::YellowRed);
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::YellowOrangeGreen);
            assert_eq!(cube.cube_elements[2][0][1], CubeElement::YellowGreen);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::YellowGreenRed);
        }

        #[test]
        fn front_clockwise() {
            let mut cube = load_solved_cube();
            cube.front_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::YellowBlueOrange);
            assert_eq!(cube.cube_elements[0][1][0], CubeElement::YellowBlue);
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[1][0][0], CubeElement::OrangeBlue);
            assert_eq!(cube.cube_elements[1][1][0], CubeElement::Blue);
            assert_eq!(cube.cube_elements[1][2][0], CubeElement::RedBlue);
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::WhiteBlueOrange);
            assert_eq!(cube.cube_elements[2][1][0], CubeElement::WhiteBlue);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::WhiteRedBlue);
        }

        #[test]
        fn front_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.front_counter_clockwise();
            assert_eq!(cube.cube_elements[0][0][0], CubeElement::WhiteRedBlue);
            assert_eq!(cube.cube_elements[0][1][0], CubeElement::WhiteBlue);
            assert_eq!(cube.cube_elements[0][2][0], CubeElement::WhiteBlueOrange);
            assert_eq!(cube.cube_elements[1][0][0], CubeElement::RedBlue);
            assert_eq!(cube.cube_elements[1][1][0], CubeElement::Blue);
            assert_eq!(cube.cube_elements[1][2][0], CubeElement::OrangeBlue);
            assert_eq!(cube.cube_elements[2][0][0], CubeElement::YellowRedBlue);
            assert_eq!(cube.cube_elements[2][1][0], CubeElement::YellowBlue);
            assert_eq!(cube.cube_elements[2][2][0], CubeElement::YellowBlueOrange);
        }

        #[test]
        fn back_clockwise() {
            let mut cube = load_solved_cube();
            cube.back_clockwise();
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::WhiteGreenRed);
            assert_eq!(cube.cube_elements[0][1][2], CubeElement::WhiteGreen);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[1][0][2], CubeElement::RedGreen);
            assert_eq!(cube.cube_elements[1][1][2], CubeElement::Green);
            assert_eq!(cube.cube_elements[1][2][2], CubeElement::OrangeGreen);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::YellowGreenRed);
            assert_eq!(cube.cube_elements[2][1][2], CubeElement::YellowGreen);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::YellowOrangeGreen);
        }

        #[test]
        fn back_counter_clockwise() {
            let mut cube = load_solved_cube();
            cube.back_counter_clockwise();
            assert_eq!(cube.cube_elements[0][0][2], CubeElement::YellowOrangeGreen);
            assert_eq!(cube.cube_elements[0][1][2], CubeElement::YellowGreen);
            assert_eq!(cube.cube_elements[0][2][2], CubeElement::YellowGreenRed);
            assert_eq!(cube.cube_elements[1][0][2], CubeElement::OrangeGreen);
            assert_eq!(cube.cube_elements[1][1][2], CubeElement::Green);
            assert_eq!(cube.cube_elements[1][2][2], CubeElement::RedGreen);
            assert_eq!(cube.cube_elements[2][0][2], CubeElement::WhiteOrangeGreen);
            assert_eq!(cube.cube_elements[2][1][2], CubeElement::WhiteGreen);
            assert_eq!(cube.cube_elements[2][2][2], CubeElement::WhiteGreenRed);
        }
        
        #[test]
        fn print() {
            let cube = load_solved_cube();
            println!("{}", cube);
        }
    }
}
